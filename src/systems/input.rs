mod collider;
mod selector;

use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;

pub use self::collider::*;
pub use self::selector::*;
use super::bio::LymphNode;
use super::draw_square;
use super::units::Unit;
use crate::pathfinding::PathseekersQueue;
use crate::ui::UiEvent;

const HIGHLIGHT_ZOOM_MAX: f32 = 1.1;
const HIGHLIGHT_ZOOM_MIN: f32 = 0.9;
const HIGHLIGHT_ZOOM_SPEED: f32 = 0.01;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputState::default())
            .add_system(track_mouse_position)
            .add_system(track_mouse_buttons)
            .add_system(track_mouse)
            .add_system(track_selector_hovers)
            .add_system(track_selector_picks)
            .add_system(animate_selectors);
    }
}

pub struct InputState {
    pub mouse_pos: Vec2,
    pub is_dragging: bool,
    pub drag_start_pos: Vec2,
    pub selected_units: Vec<Entity>,
    pub selected_units_dirty: bool,
    pub hovered_entity: Option<Entity>,
    pub highlight_zoom: f32,
    pub highlight_zoom_dir: f32,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            is_dragging: false,
            drag_start_pos: Vec2::ZERO,
            selected_units: Default::default(),
            selected_units_dirty: false,
            hovered_entity: Default::default(),
            highlight_zoom: 1.0,
            highlight_zoom_dir: 1.0,
        }
    }
}

fn track_mouse_position(
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<InputState>,
    wnds: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        return;
    }

    let (camera, camera_transform) = camera.single();
    let wnd = wnds.get(camera.window).unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix()
            * camera.projection_matrix.inverse();

        state.mouse_pos =
            ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
    }
}

fn track_mouse_buttons(
    mut egui: ResMut<EguiContext>,
    state: Res<InputState>,
    mut pathfinding_queue: ResMut<PathseekersQueue>,
    mouse: Res<Input<MouseButton>>,
    mut units: Query<(Entity, &mut Unit, &Transform)>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        return;
    }

    if mouse.just_pressed(MouseButton::Right) {
        for unit in state.selected_units.iter() {
            if let Ok((entity, mut unit, transform)) = units.get_mut(*unit) {
                let target = state.mouse_pos;

                pathfinding_queue.insert(
                    entity,
                    transform.translation.truncate(),
                    target,
                );

                unit.target = Some(target);
            }
        }
    }
}

fn track_mouse(
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<InputState>,
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
        state.drag_start_pos = state.mouse_pos;
    } else if state.is_dragging && mouse.just_released(MouseButton::Left) {
        state.is_dragging = false;

        let drag_x = (state.mouse_pos.x - state.drag_start_pos.x).abs();
        let drag_y = (state.mouse_pos.y - state.drag_start_pos.y).abs();
        let size = 50.0; // TODO(pry): this info should be within unit struct
        let offset = Vec2::new(size, size);

        if drag_x > offset.x && drag_y > offset.y {
            state.selected_units = units
                .iter()
                .filter(|(_, transform, _)| {
                    point_in_rect(
                        transform.translation.truncate(),
                        state.drag_start_pos,
                        state.mouse_pos,
                    )
                })
                .map(|(entity, _, _)| entity)
                .collect();

            state.selected_units_dirty = true;
        } else {
            let start_pos = state.mouse_pos + offset;
            let end_pos = state.mouse_pos - offset;

            state.selected_units = units
                .iter()
                .filter(|(_, transform, _)| {
                    point_in_rect(
                        transform.translation.truncate(),
                        start_pos,
                        end_pos,
                    )
                })
                .sorted_by(|(_, a, _), (_, b, _)| {
                    let a = a.translation.truncate().distance(state.mouse_pos);
                    let b = b.translation.truncate().distance(state.mouse_pos);

                    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
                })
                .map(|(entity, _, _)| entity)
                .next()
                .into_iter()
                .collect();

            state.selected_units_dirty = true;
        }
    }

    if state.is_dragging {
        draw_square(&mut lines, state.drag_start_pos, state.mouse_pos);
    }
}

fn track_selector_hovers(
    mut state: ResMut<InputState>,
    entities: Query<(Entity, &Collider, &Transform, &Children)>,
    mut selectors: Query<&mut Selector>,
) {
    if state.is_dragging {
        // looks kiczowato
        return;
    }

    let new_hovered_entity =
        entities
            .iter()
            .find_map(|(entity, collider, transform, children)| {
                if collider
                    .contains(transform.translation.truncate(), state.mouse_pos)
                {
                    Some((entity, children))
                } else {
                    None
                }
            });

    if new_hovered_entity.map(|(e, _)| e) != state.hovered_entity {
        if let Some(old_hovered_entity) = state.hovered_entity {
            if let Ok((_, _, _, children)) = entities.get(old_hovered_entity) {
                Selector::modify(&mut selectors, children, |selector| {
                    selector.hovered = false;
                });
            }
        }

        if let Some((_, children)) = new_hovered_entity {
            Selector::modify(&mut selectors, children, |selector| {
                selector.hovered = true;
            });
        }

        state.hovered_entity = new_hovered_entity.map(|(e, _)| e);
    }
}

fn track_selector_picks(
    mut state: ResMut<InputState>,
    units: Query<(Entity, &Unit, &Children)>,
    mut selectors: Query<&mut Selector>,
) {
    if !state.selected_units_dirty {
        return;
    }

    for (entity, _, children) in units.iter() {
        for &child in children.iter() {
            if let Ok(mut selector) = selectors.get_mut(child) {
                selector.picked = state.selected_units.contains(&entity);
            }
        }
    }

    state.selected_units_dirty = false;
}

fn animate_selectors(
    mut state: ResMut<InputState>,
    mut selectors: Query<(&Selector, &mut Visibility, &mut Transform)>,
) {
    if state.highlight_zoom + HIGHLIGHT_ZOOM_SPEED > HIGHLIGHT_ZOOM_MAX {
        state.highlight_zoom_dir = -1.0
    } else if state.highlight_zoom - HIGHLIGHT_ZOOM_SPEED < HIGHLIGHT_ZOOM_MIN {
        state.highlight_zoom_dir = 1.0
    }

    state.highlight_zoom += HIGHLIGHT_ZOOM_SPEED * state.highlight_zoom_dir;

    for (selector, mut visibility, mut transform) in selectors.iter_mut() {
        visibility.is_visible = selector.hovered || selector.picked;

        if selector.picked {
            transform.scale.x = state.highlight_zoom;
            transform.scale.y = state.highlight_zoom;
        } else {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        }
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
