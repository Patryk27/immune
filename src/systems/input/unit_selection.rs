use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_prototype_debug_lines::DebugLines;

use super::{draw_square, InputState, MousePos};
use crate::systems::bio::LymphNode;
use crate::systems::units::Unit;
use crate::ui::UiEvent;

const MIN_DRAG_DISTANCE: f32 = 1.0;

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedUnits::default())
            .insert_resource(SelectionGroups::default())
            .add_event::<SelectedUnitsChanged>()
            .add_event::<Selection>()
            .add_system(unit_selection)
            .add_system(track_drag)
            .add_system(group_selection)
            .add_system(draw_selection_rect)
            .add_system(select_lymph_node);
    }
}

#[derive(Default)]
pub struct SelectedUnits {
    pub selected_units: HashSet<Entity>,
}

pub struct SelectedUnitsChanged;

pub enum Selection {
    Drag,
    Click,
}

#[derive(Default)]
pub struct SelectionGroups(pub HashMap<usize, HashSet<Entity>>);

fn draw_selection_rect(
    state: Res<InputState>,
    mouse_pos: Res<MousePos>,
    mut lines: ResMut<DebugLines>,
) {
    if state.is_dragging {
        draw_square(&mut lines, state.drag_start_pos, mouse_pos.0);
    }
}

fn group_selection(
    keyboard: Res<Input<KeyCode>>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selection_groups: ResMut<SelectionGroups>,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
) {
    if let Some(group) =
        which_group_key_is_pressed(&keyboard, Input::just_pressed)
    {
        let group = selection_groups.0.entry(group).or_default();

        if keyboard.pressed(KeyCode::LControl) {
            *group = selected_units.selected_units.clone();
        } else {
            selected_units.selected_units = group.clone();
            selected_units_changed.send(SelectedUnitsChanged);
        }
    }
}

fn which_group_key_is_pressed(
    keyboard: &Input<KeyCode>,
    method: impl Fn(&Input<KeyCode>, KeyCode) -> bool,
) -> Option<usize> {
    const GROUP_KEYS: &[KeyCode] = &[
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ];

    for (idx, key) in GROUP_KEYS.iter().enumerate() {
        if method(keyboard, *key) {
            return Some(idx + 1);
        }
    }

    None
}

fn track_drag(
    mut mouse_pos_at_start_of_drag: Local<Vec2>,
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<InputState>,
    mouse_pos: Res<MousePos>,
    mouse: Res<Input<MouseButton>>,
    mut selection_event: EventWriter<Selection>,
) {
    let is_pointer_over_ui = egui.ctx_mut().is_pointer_over_area();

    // We can still stop dragging if over ui
    if mouse.just_released(MouseButton::Left) {
        if state.is_dragging {
            state.is_dragging = false;

            selection_event.send(Selection::Drag);
        } else if !is_pointer_over_ui {
            selection_event.send(Selection::Click);
        }

        return;
    }

    if is_pointer_over_ui {
        return;
    }

    if !state.is_dragging && mouse.just_pressed(MouseButton::Left) {
        *mouse_pos_at_start_of_drag = mouse_pos.0;

        return;
    }

    if !state.is_dragging
        && mouse.pressed(MouseButton::Left)
        && mouse_pos_at_start_of_drag.distance(mouse_pos.0) > MIN_DRAG_DISTANCE
    {
        state.is_dragging = true;
        state.drag_start_pos = *mouse_pos_at_start_of_drag;

        return;
    }
}

fn select_lymph_node(
    state: Res<InputState>,
    lymph_nodes: Query<(), With<LymphNode>>,
    mut selection_event: EventReader<Selection>,
    mut ui_events: EventWriter<UiEvent>,
) {
    if let Some(Selection::Click) = selection_event.iter().next() {
        if let Some(entity) = state.hovered_entity {
            if lymph_nodes.get(entity).is_ok() {
                ui_events.send(UiEvent::LymphNodeClicked(entity));
                return;
            }
        }
    }
}

fn unit_selection(
    state: Res<InputState>,
    mouse_pos: Res<MousePos>,
    keyboard: Res<Input<KeyCode>>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
    units: Query<(Entity, &Transform, &Unit)>,
    mut selection_event: EventReader<Selection>,
) {
    match selection_event.iter().next() {
        Some(Selection::Drag) => selected_units_in_rect(
            &units,
            &keyboard,
            state.drag_start_pos,
            mouse_pos.0,
            &mut selected_units,
            &mut selected_units_changed,
        ),
        Some(Selection::Click) => {
            select_unit_at_point(
                &state,
                &keyboard,
                units,
                &mut selected_units,
                selected_units_changed,
            );
        }
        _ => (),
    }
}

fn selected_units_in_rect(
    units: &Query<(Entity, &Transform, &Unit)>,
    keyboard: &Input<KeyCode>,
    drag_start_pos: Vec2,
    mouse_pos: Vec2,
    selected_units: &mut SelectedUnits,
    selected_units_changed: &mut EventWriter<SelectedUnitsChanged>,
) {
    let new_selected_units = units
        .iter()
        .filter(|(_, transform, _)| {
            point_in_rect(
                transform.translation.truncate(),
                drag_start_pos,
                mouse_pos,
            )
        })
        .map(|(entity, _, _)| entity);

    if is_appending_selection(keyboard) {
        selected_units.selected_units.extend(new_selected_units);
    } else {
        selected_units.selected_units = new_selected_units.collect();
    }

    selected_units_changed.send(SelectedUnitsChanged);
}

fn select_unit_at_point(
    state: &InputState,
    keyboard: &Input<KeyCode>,
    units: Query<(Entity, &Transform, &Unit)>,
    selected_units: &mut SelectedUnits,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
) {
    if let Some(entity) = state.hovered_entity {
        if units.get(entity).is_ok() {
            if !is_appending_selection(keyboard) {
                selected_units.selected_units.clear();
            }

            selected_units.selected_units.insert(entity);

            selected_units_changed.send(SelectedUnitsChanged);
        }
    } else {
        selected_units.selected_units.clear();

        selected_units_changed.send(SelectedUnitsChanged);
    }
}

fn is_appending_selection(keyboard: &Input<KeyCode>) -> bool {
    keyboard.pressed(KeyCode::LControl)
}

fn point_in_rect(
    point: Vec2,
    rect_top_left: Vec2,
    rect_bottom_right: Vec2,
) -> bool {
    let min_x = rect_top_left.x.min(rect_bottom_right.x);
    let max_x = rect_top_left.x.max(rect_bottom_right.x);
    let min_y = rect_top_left.y.min(rect_bottom_right.y);
    let max_y = rect_top_left.y.max(rect_bottom_right.y);

    point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y
}
