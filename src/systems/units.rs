use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::physics::pixel_to_world;

const MAX_SPEED: f32 = 5.0;
const FORCE_FACTOR: f32 = 1.0;
const STOPPING_FORCE_FACTOR: f32 = 2.0;

#[derive(Component, Default)]
pub struct Unit {
    // TODO(dzejkop): Should be enum, target can be unit, etc.
    pub target: Option<Vec2>,
    pub path: Vec<Vec2>,
    pub step: usize,
}

impl Unit {
    pub fn set_path(&mut self, path: Vec<Vec2>, target: Option<Vec2>) {
        self.step = 0;
        self.path = path;
        self.target = target;
    }
}

pub fn initialize(app: &mut App) {
    app.add_system(movement);
}

pub fn movement(
    mut units: Query<(
        &RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &mut Unit,
        &Transform
    )>,
) {
    for (velocity, mut forces, mut unit, transform) in units.iter_mut() {
        if let Some(target) = unit.target {
            let current_pos = transform.translation.truncate();
            let default_force_direction = target - current_pos;

            let force_direction = if let Some(_) = unit.path.get(unit.step).copied() {
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
            } else {
                default_force_direction
            };

            let force_direction = pixel_to_world(force_direction);
            let desired_linvel: Vector<Real> = if force_direction.magnitude() < 1.0 {
                force_direction * MAX_SPEED
            } else {
                force_direction.normalize() * MAX_SPEED
            };

            let current_linvel = velocity.linvel;

            let diff = desired_linvel - current_linvel;

            forces.force = diff * FORCE_FACTOR;
        } else {
            // Try to stop moving
            let desired_linvel: Vector<Real> = [0.0, 0.0].into();
            let current_linvel = velocity.linvel;

            let diff = desired_linvel - current_linvel;
            forces.force = diff * STOPPING_FORCE_FACTOR;
        }
    }
}

fn rotate(v: Vec2, to: Vec2) -> Vec2 {
    let angle = to.angle_between(v);
    let x = v.x * angle.cos() - v.y * angle.sin();
    let y = v.x * angle.sin() + v.y * angle.cos();

    Vec2::new(x, y)
}
