use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Unit, FORCE_FACTOR, MAX_SPEED, STOPPING_FORCE_FACTOR};
use crate::systems::physics::pixel_to_world;

pub fn system(
    mut units: Query<(
        &RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &mut Unit,
        &Transform,
    )>,
) {
    for (velocity, mut forces, mut unit, transform) in units.iter_mut() {
        if let Some(target) = unit.target {
            move_towards_target(
                transform,
                target,
                &mut unit,
                velocity,
                &mut forces,
            );
        } else {
            maintain_position(velocity, &mut forces);
        }
    }
}

fn move_towards_target(
    transform: &Transform,
    target: Vec2,
    unit: &mut Unit,
    velocity: &RigidBodyVelocityComponent,
    forces: &mut RigidBodyForcesComponent,
) {
    let current_pos = transform.translation.truncate();

    let default_force_direction = target - current_pos;

    let force_direction =
        force_direction_from_path(unit, current_pos, default_force_direction);
    let force_direction = pixel_to_world(force_direction);

    let desired_linvel: Vector<Real> = if force_direction.magnitude() < 1.0 {
        force_direction * MAX_SPEED
    } else {
        force_direction.normalize() * MAX_SPEED
    };

    let current_linvel = velocity.linvel;
    let diff = desired_linvel - current_linvel;

    forces.force = diff * FORCE_FACTOR;
}

fn force_direction_from_path(
    unit: &mut Unit,
    current_pos: Vec2,
    default_force_direction: Vec2,
) -> Vec2 {
    if unit.path.get(unit.step).is_none() {
        return Vec2::ZERO;
    }

    let mut min_distance_from_node = f32::INFINITY;
    let mut new_step = unit.step;

    // Follow through path
    for (idx, pos) in unit.path.iter().enumerate().skip(unit.step) {
        let actual_distance_from_node = pos.distance(current_pos);

        if min_distance_from_node > actual_distance_from_node {
            new_step = idx;
            min_distance_from_node = actual_distance_from_node
        }
    }

    unit.step = new_step;

    if unit.step < unit.path.len() - 1 {
        let next_pos = unit.path[unit.step + 1];
        let path_direction = next_pos - current_pos;

        // Compensate direction by path
        rotate(default_force_direction, path_direction)
    } else {
        default_force_direction
    }
}

fn maintain_position(
    velocity: &RigidBodyVelocityComponent,
    forces: &mut RigidBodyForcesComponent,
) {
    let desired_linvel: Vector<Real> = [0.0, 0.0].into();
    let current_linvel = velocity.linvel;

    let diff = desired_linvel - current_linvel;

    forces.force = diff * STOPPING_FORCE_FACTOR;
}

fn rotate(v: Vec2, to: Vec2) -> Vec2 {
    let angle = to.angle_between(v);
    let x = v.x * angle.cos() - v.y * angle.sin();
    let y = v.x * angle.sin() + v.y * angle.cos();

    Vec2::new(x, y)
}
