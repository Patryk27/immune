use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

use super::physics::world_to_pixel;

pub fn initialize(app: &mut App) {
    app.insert_resource(DebugState::default())
        .add_system(draw_motion_vectors);
}

pub struct DebugState {
    pub show_debug_window: bool,
    pub show_motion_vectors: bool,
    pub show_force_vectors: bool,
    pub track_position: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_debug_window: true,
            show_motion_vectors: false,
            show_force_vectors: false,
            track_position: false,
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
