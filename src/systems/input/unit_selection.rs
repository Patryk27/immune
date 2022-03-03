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
            .add_event::<SelectedUnitsChanged>()
            .add_event::<Selection>()
            .add_system(unit_selection)
            .add_system(track_drag)
            .add_system(draw_selection_rect)
            .add_system(select_lymph_node);
    }
}

#[derive(Default)]
pub struct SelectedUnits {
    pub selected_units: Vec<Entity>,
}

pub struct SelectedUnitsChanged;

pub enum Selection {
    Drag,
    Click,
}

fn draw_selection_rect(
    state: Res<InputState>,
    mouse_pos: Res<MousePos>,
    mut lines: ResMut<DebugLines>,
) {
    if state.is_dragging {
        draw_square(&mut lines, state.drag_start_pos, mouse_pos.0);
    }
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
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
    units: Query<(Entity, &Transform, &Unit)>,
    mut selection_event: EventReader<Selection>,
) {
    match selection_event.iter().next() {
        Some(Selection::Drag) => selected_units_in_rect(
            &units,
            state.drag_start_pos,
            mouse_pos.0,
            &mut selected_units,
            &mut selected_units_changed,
        ),
        Some(Selection::Click) => {
            select_unit_at_point(
                &state,
                units,
                &mut selected_units,
                selected_units_changed,
            );
        }
        _ => (),
    }
}

fn select_unit_at_point(
    state: &InputState,
    units: Query<(Entity, &Transform, &Unit)>,
    selected_units: &mut SelectedUnits,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
) {
    if let Some(entity) = state.hovered_entity {
        if units.get(entity).is_ok() {
            selected_units.selected_units = vec![entity];

            selected_units_changed.send(SelectedUnitsChanged);
        }
    } else {
        selected_units.selected_units = vec![];

        selected_units_changed.send(SelectedUnitsChanged);
    }
}

fn selected_units_in_rect(
    units: &Query<(Entity, &Transform, &Unit)>,
    drag_start_pos: Vec2,
    mouse_pos: Vec2,
    selected_units: &mut SelectedUnits,
    selected_units_changed: &mut EventWriter<SelectedUnitsChanged>,
) {
    selected_units.selected_units = units
        .iter()
        .filter(|(_, transform, _)| {
            point_in_rect(
                transform.translation.truncate(),
                drag_start_pos,
                mouse_pos,
            )
        })
        .map(|(entity, _, _)| entity)
        .collect();

    selected_units_changed.send(SelectedUnitsChanged);
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
