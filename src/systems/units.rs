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
}

pub fn initialize(app: &mut App) {
    app.add_system(movement);
}

pub fn movement(
    mut units: Query<(
        &RigidBodyPositionComponent,
        &RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &Unit,
    )>,
) {
    for (position, velocity, mut forces, unit) in units.iter_mut() {
        if let Some(target) = unit.target {
            let target: Vector<Real> = pixel_to_world(target);
            let source = position.position.translation.vector;

            let to_target = target - source;
            let desired_linvel: Vector<Real> = if to_target.magnitude() < 1.0 {
                to_target * MAX_SPEED
            } else {
                to_target.normalize() * MAX_SPEED
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
