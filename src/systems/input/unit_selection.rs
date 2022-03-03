use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;

use super::{
    draw_square, InputState, MousePos, SelectedUnits, SelectedUnitsChanged,
};
use crate::systems::bio::LymphNode;
use crate::systems::units::Unit;
use crate::ui::UiEvent;

pub fn unit_selection(
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<InputState>,
    mouse_pos: Res<MousePos>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
    mouse: Res<Input<MouseButton>>,
    units: Query<(Entity, &Transform, &Unit)>,
    lymph_nodes: Query<(), With<LymphNode>>,
    mut lines: ResMut<DebugLines>,
    mut ui_events: EventWriter<UiEvent>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        return;
    }

    if !state.is_dragging && mouse.just_pressed(MouseButton::Left) {
        if let Some(entity) = state.hovered_entity {
            if lymph_nodes.get(entity).is_ok() {
                ui_events.send(UiEvent::LymphNodeClicked(entity));
                return;
            }
        }
    }

    if !state.is_dragging && mouse.just_pressed(MouseButton::Left) {
        state.is_dragging = true;
        state.drag_start_pos = mouse_pos.0;
    } else if state.is_dragging && mouse.just_released(MouseButton::Left) {
        state.is_dragging = false;

        let drag_x = (mouse_pos.0.x - state.drag_start_pos.x).abs();
        let drag_y = (mouse_pos.0.y - state.drag_start_pos.y).abs();
        let size = 50.0; // TODO(pry): this info should be within unit struct
        let offset = Vec2::new(size, size);

        if drag_x > offset.x && drag_y > offset.y {
            selected_units.selected_units = units
                .iter()
                .filter(|(_, transform, _)| {
                    point_in_rect(
                        transform.translation.truncate(),
                        state.drag_start_pos,
                        mouse_pos.0,
                    )
                })
                .map(|(entity, _, _)| entity)
                .collect();

            selected_units_changed.send(SelectedUnitsChanged);
        } else {
            let start_pos = mouse_pos.0 + offset;
            let end_pos = mouse_pos.0 - offset;

            selected_units.selected_units = units
                .iter()
                .filter(|(_, transform, _)| {
                    point_in_rect(
                        transform.translation.truncate(),
                        start_pos,
                        end_pos,
                    )
                })
                .sorted_by(|(_, a, _), (_, b, _)| {
                    let a = a.translation.truncate().distance(mouse_pos.0);
                    let b = b.translation.truncate().distance(mouse_pos.0);

                    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
                })
                .map(|(entity, _, _)| entity)
                .next()
                .into_iter()
                .collect();

            selected_units_changed.send(SelectedUnitsChanged);
        }
    }

    if state.is_dragging {
        draw_square(&mut lines, state.drag_start_pos, mouse_pos.0);
    }
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
