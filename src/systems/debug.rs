use std::f32::consts::PI;

use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

use super::draw_square;
use super::input::InputState;
use super::physics::world_to_pixel;
use super::units::Unit;
use crate::pathfinding::{DiscreteMap, Map, FIELD_SIZE};
use crate::systems::draw_square_dur;

pub const DEBUG_MAP_FIELD_SIZE: f32 = 5.0;

pub fn initialize(app: &mut App) {
    app.insert_resource(DebugState::default())
        .add_system(draw_motion_vectors)
        .add_system(capture_map)
        .add_system(draw_paths);
}

pub struct DebugState {
    pub show_debug_window: bool,
    pub show_motion_vectors: bool,
    pub show_force_vectors: bool,
    pub show_pathfinding: bool,
    pub track_position: bool,
    pub draw_obstacles_from_map: bool,
    pub is_dragging: bool,
    pub drag_start_pos: Vec2,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_debug_window: true,
            show_motion_vectors: false,
            show_force_vectors: false,
            show_pathfinding: false,
            track_position: false,
            draw_obstacles_from_map: false,
            is_dragging: false,
            drag_start_pos: Vec2::ZERO,
        }
    }
}

pub fn draw_paths(
    state: Res<DebugState>,
    query: Query<(&Transform, &Unit)>,
    mut lines: ResMut<DebugLines>,
) {
    if !state.show_pathfinding {
        return;
    }

    for (transform, unit) in query.iter() {
        let mut prev = transform.translation;

        if let Some(target) = unit.target {
            draw_arrow(&mut lines, prev, target.extend(0.0), Color::TOMATO);
        }

        if unit.path.is_empty() {
            continue;
        }

        for p in &unit.path {
            let p = p.extend(0.0);
            draw_arrow(&mut lines, prev, p, Color::FUCHSIA);
            prev = p;
        }
    }
}

pub fn draw_motion_vectors(
    state: Res<DebugState>,
    query: Query<(
        &RigidBodyPositionComponent,
        &RigidBodyVelocityComponent,
        &RigidBodyForcesComponent,
    )>,
    mut lines: ResMut<DebugLines>,
) {
    if !state.show_motion_vectors
        && !state.show_force_vectors
        && !state.track_position
    {
        return;
    }

    for (position, velocity, force) in query.iter() {
        let position = position.position.translation.vector;
        let velocity = velocity.linvel;
        let force = force.force;

        if state.show_force_vectors {
            let color = Color::rgb(1.0, 0.0, 0.0);

            let start_pos = world_to_pixel(position);
            let end_pos = world_to_pixel(position + force);

            draw_arrow(&mut lines, start_pos, end_pos, color);
        }

        if state.track_position {
            let pos = world_to_pixel(position);

            lines.line(pos, pos + Vec3::X, 1.0);
        }

        if state.show_motion_vectors {
            show_velocity_vector(position, velocity, &mut lines);
        }
    }
}

pub fn capture_map(
    input_state: Res<InputState>,
    mut debug_state: ResMut<DebugState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut lines: ResMut<DebugLines>,
    map: Res<Map>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Middle {
            match (debug_state.is_dragging, event.state.is_pressed()) {
                // Drag end
                (true, false)
                    if mouse_button_input
                        .just_released(MouseButton::Middle) =>
                {
                    debug_state.is_dragging = false;
                    let start = debug_state.drag_start_pos;
                    let end = input_state.mouse_pos;
                    let mid = Vec2::new(
                        (start.x + end.x) / 2f32,
                        (start.y + end.y) / 2f32,
                    );
                    let map = DiscreteMap::new(&map, mid, end);

                    println!("{map}");

                    if debug_state.draw_obstacles_from_map {
                        for pos in map.obstacles() {
                            // let field_size = FIELD_SIZE as f32 * 2f32.sqrt() / 2f32;
                            let field_size = DEBUG_MAP_FIELD_SIZE;

                            let top_left = pos - field_size;
                            let bottom_right = pos + field_size;

                            draw_square_dur(
                                &mut lines,
                                top_left,
                                bottom_right,
                                10.0,
                            );
                        }
                    }
                }
                // Drag start
                (false, true) => {
                    debug_state.is_dragging = true;
                    debug_state.drag_start_pos = input_state.mouse_pos;
                }
                _ => (),
            }
        }
    }

    if !debug_state.is_dragging {
        return;
    }

    draw_square(
        &mut lines,
        debug_state.drag_start_pos,
        input_state.mouse_pos,
    );
}

fn show_velocity_vector(
    position: Vector<Real>,
    velocity: Vector<Real>,
    lines: &mut ResMut<DebugLines>,
) {
    let color = Color::rgb(0.0, 1.0, 0.0);

    let start_pos = world_to_pixel(position);
    let end_pos = world_to_pixel(position + velocity);

    draw_arrow(lines, start_pos, end_pos, color);
}

fn draw_arrow(lines: &mut DebugLines, start: Vec3, end: Vec3, color: Color) {
    const WING_LENGTH: f32 = 10.0;

    let forward = (end - start).normalize() * WING_LENGTH;
    let angle = PI * 3.0 / 4.0;
    let left = Quat::from_axis_angle(Vec3::Z, angle);
    let right = Quat::from_axis_angle(Vec3::Z, -angle);

    let left = left * forward;
    let right = right * forward;

    lines.line_colored(start, end, 0.0, color);
    lines.line_colored(end, end + left, 0.0, color);
    lines.line_colored(end, end + right, 0.0, color);
}
