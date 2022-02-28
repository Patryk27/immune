use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;

use super::camera::screen_to_pixel;
use super::cell_node::LymphNode;
use super::highlight::Highlight;
use super::units::Unit;
use crate::pathfinding::{DiscreteMap, Map};
use crate::ui::UiEvent;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputState::default())
            .add_system(track_mouse_position)
            .add_system(process_mouse_selection)
            .add_system(highlight_selection)
            .add_system(process_mouse_command);
    }
}

struct InputState {
    mouse_pos: Vec2,
    is_dragging: bool,
    drag_start_pos: Vec2,
    selected_units: Vec<Entity>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            is_dragging: false,
            drag_start_pos: Vec2::ZERO,
            selected_units: vec![],
        }
    }
}

fn track_mouse_position(
    mut state: ResMut<InputState>,
    camera: Query<(&OrthographicProjection, &Transform), With<Camera>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let (ortho, camera) = camera.single();

    for event in cursor_moved_events.iter() {
        let pos = Vec3::new(event.position.x, event.position.y, 0.0);
        state.mouse_pos = screen_to_pixel(camera, ortho, pos).truncate();
    }
}

fn process_mouse_command(
    state: Res<InputState>,
    map: Res<Map>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut units: Query<(&mut Unit, &Transform)>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state.is_pressed() && event.button == MouseButton::Right {
            for unit in state.selected_units.iter() {
                let (mut unit, transform) = units.get_mut(*unit).unwrap();
                unit.target = Some(state.mouse_pos);
                let discrete_map = DiscreteMap::new(
                    &map,
                    transform.translation.truncate(),
                    state.mouse_pos,
                );
                // TODO (pry)
                // println!("{}", discrete_map);
            }
        }
    }
}

fn process_mouse_selection(
    mut state: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mouse_button_input: Res<Input<MouseButton>>,
    units: Query<(Entity, &Transform, &Unit)>,
    lymph_nodes: Query<(Entity, &Transform, &LymphNode)>,
    mut lines: ResMut<DebugLines>,
    mut ui_events: EventWriter<UiEvent>,
) {
    if !state.is_dragging && mouse_button_input.just_pressed(MouseButton::Left)
    {
        let clicked_entity =
            lymph_nodes.iter().find_map(|(entity, transform, _)| {
                if point_in_circle(
                    state.mouse_pos,
                    transform.translation.truncate(),
                    70.0,
                ) {
                    Some(entity)
                } else {
                    None
                }
            });

        if let Some(entity) = clicked_entity {
            ui_events.send(UiEvent::OpenLymphNodeEditor(entity));
            return;
        }
    }

    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            match (state.is_dragging, event.state.is_pressed()) {
                // Drag end
                (true, false)
                    if mouse_button_input.just_released(MouseButton::Left) =>
                {
                    state.is_dragging = false;

                    let drag_x =
                        (state.mouse_pos.x - state.drag_start_pos.x).abs();
                    let drag_y =
                        (state.mouse_pos.y - state.drag_start_pos.y).abs();
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
                            .sorted_by(|(_, one, _), (_, other, _)| {
                                let one_distance = (one
                                    .translation
                                    .truncate()
                                    .distance(state.mouse_pos)
                                    * 100.0)
                                    as u64;

                                let other_distance = (other
                                    .translation
                                    .truncate()
                                    .distance(state.mouse_pos)
                                    * 100.0)
                                    as u64;

                                one_distance.cmp(&other_distance)
                            })
                            .map(|(entity, _, _)| entity)
                            .next()
                            .into_iter()
                            .collect();
                    }
                }
                // Drag start
                (false, true) => {
                    state.is_dragging = true;
                    state.drag_start_pos = state.mouse_pos;
                }
                _ => (),
            }
        }
    }

    if !state.is_dragging {
        return;
    }

    draw_square(&mut lines, state.drag_start_pos, state.mouse_pos);
}

fn highlight_selection(
    state: ResMut<InputState>,
    units: Query<(Entity, &Unit, &Children)>,
    mut highlights: Query<(Entity, &Highlight, &mut Visibility)>,
) {
    // Has to be greater than max number of children of Unit to allocate only once
    let unit_children = 10;
    let capacity = state.selected_units.len() * unit_children;
    let mut selected_children: Vec<Entity> = Vec::with_capacity(capacity);

    for selected_entity in state.selected_units.iter() {
        let (_, _, children) = units.get(*selected_entity).unwrap();

        selected_children.extend(children.iter());
    }

    for (entity, _, mut visibility) in highlights.iter_mut() {
        visibility.is_visible = selected_children.contains(&entity);
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

fn point_in_circle(
    point: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
) -> bool {
    point.distance(circle_center) <= circle_radius
}

fn draw_square(lines: &mut DebugLines, start_point: Vec2, end_point: Vec2) {
    let start_point = start_point.extend(0.0);
    let end_point = end_point.extend(0.0);
    let right = Vec3::new(end_point.x - start_point.x, 0.0, 0.0);
    let up = Vec3::new(0.0, end_point.y - start_point.y, 0.0);

    lines.line(start_point, start_point + right, 0.0);
    lines.line(start_point, start_point + up, 0.0);
    lines.line(end_point, end_point - right, 0.0);
    lines.line(end_point, end_point - up, 0.0);
}
