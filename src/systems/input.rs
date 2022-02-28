use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;

use super::cell_node::LymphNode;
use super::highlight::Highlight;
use super::units::Unit;
use crate::map::Map;
use crate::pathfinding::DiscreteMap;
use crate::ui::UiEvent;

pub struct State {
    selected_units: Vec<Entity>,

    is_dragging: bool,
    drag_start_pos: Vec3,

    pub current_mouse_pos: Vec3,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected_units: vec![],
            is_dragging: false,
            current_mouse_pos: Vec3::ZERO,
            drag_start_pos: Vec3::ZERO,
        }
    }
}

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(track_mouse_position)
        .add_system(selection)
        .add_system(highlight_selection)
        .add_system(command);
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(State::default());
}

pub fn track_mouse_position(
    mut state: ResMut<State>,
    camera: Query<(&OrthographicProjection, &Camera, &Transform)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let (ortho, _, camera) = camera.single();

    for event in cursor_moved_events.iter() {
        let pos = Vec3::new(event.position.x, event.position.y, 0.0);
        state.current_mouse_pos = screen_to_world_point(camera, ortho, pos);
    }
}

pub fn command(
    state: Res<State>,
    map: Res<Map>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut units: Query<(&mut Unit, &Transform)>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state.is_pressed() && event.button == MouseButton::Right {
            for unit in state.selected_units.iter() {
                let (mut unit, transform) = units.get_mut(*unit).unwrap();
                unit.target = Some(state.current_mouse_pos);
                let discrete_map = DiscreteMap::new(
                    &map,
                    transform.translation.truncate(),
                    state.current_mouse_pos.truncate(),
                );
                // TODO (pry)
                // println!("{}", discrete_map);
            }
        }
    }
}

pub fn selection(
    mut state: ResMut<State>,
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
                let pos = transform.translation.truncate();

                // TODO(pwy) feels like those shouldn't be hardcoded
                let bb = Rect {
                    left: pos.x - 50.0,
                    right: pos.x + 50.0,
                    top: pos.y - 50.0,
                    bottom: pos.y + 50.0,
                };

                let mouse = state.current_mouse_pos;

                let is_clicked = mouse.x >= bb.left
                    && mouse.x <= bb.right
                    && mouse.y >= bb.top
                    && mouse.y <= bb.bottom;

                if is_clicked {
                    Some(entity)
                } else {
                    None
                }
            });

        if let Some(entity) = clicked_entity {
            ui_events.send(UiEvent::OpenLymphNodeEditor(entity));
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

                    let drag_x = (state.current_mouse_pos.x
                        - state.drag_start_pos.x)
                        .abs();
                    let drag_y = (state.current_mouse_pos.y
                        - state.drag_start_pos.y)
                        .abs();
                    let size = 50.0; // TODO(pry): this info should be within unit struct
                    let offset = Vec3::new(size, size, 0.0);

                    if drag_x > offset.x && drag_y > offset.y {
                        state.selected_units = units
                            .iter()
                            .filter(|(_, transform, _)| {
                                is_unit_within_selection(
                                    transform,
                                    state.drag_start_pos,
                                    state.current_mouse_pos,
                                )
                            })
                            .map(|(entity, _, _)| entity)
                            .collect();
                    } else {
                        let start_pos = state.current_mouse_pos + offset;
                        let end_pos = state.current_mouse_pos - offset;

                        state.selected_units = units
                            .iter()
                            .filter(|(_, transform, _)| {
                                is_unit_within_selection(
                                    transform, start_pos, end_pos,
                                )
                            })
                            .sorted_by(|(_, one, _), (_, other, _)| {
                                let one_distance = (one
                                    .translation
                                    .distance(state.current_mouse_pos)
                                    * 100.0)
                                    as u64;
                                let other_distance = (other
                                    .translation
                                    .distance(state.current_mouse_pos)
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
                    state.drag_start_pos = state.current_mouse_pos;
                }
                _ => (),
            }
        }
    }

    if !state.is_dragging {
        return;
    }

    draw_square(&mut lines, state.drag_start_pos, state.current_mouse_pos);
}

pub fn highlight_selection(
    state: ResMut<State>,
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
        if selected_children.contains(&entity) {
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}

fn is_unit_within_selection(
    unit: &Transform,
    start_pos: Vec3,
    end_pos: Vec3,
) -> bool {
    let min_x = start_pos.x.min(end_pos.x);
    let max_x = start_pos.x.max(end_pos.x);
    let min_y = start_pos.y.min(end_pos.y);
    let max_y = start_pos.y.max(end_pos.y);

    unit.translation.x > min_x
        && unit.translation.x < max_x
        && unit.translation.y > min_y
        && unit.translation.y < max_y
}

fn draw_square(lines: &mut DebugLines, start_point: Vec3, end_point: Vec3) {
    let right = Vec3::new(end_point.x - start_point.x, 0.0, 0.0);
    let up = Vec3::new(0.0, end_point.y - start_point.y, 0.0);

    lines.line(start_point, start_point + right, 0.0);
    lines.line(start_point, start_point + up, 0.0);
    lines.line(end_point, end_point - right, 0.0);
    lines.line(end_point, end_point - up, 0.0);
}

// Probably not compatible with zooming or other projections
fn screen_to_world_point(
    camera: &Transform,
    ortho: &OrthographicProjection,
    point: Vec3,
) -> Vec3 {
    let ortho_offset = Vec3::new(ortho.left, ortho.bottom, 0.0);

    point + camera.translation + ortho_offset
}
