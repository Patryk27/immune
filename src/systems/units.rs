use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SPEED: f32 = 5000.0;

#[derive(Component, Default)]
pub struct Unit {
    // TODO(dzejkop): Should be enum, target can be unit, etc.
    pub target: Option<Vec3>,
}

pub fn initialize(app: &mut App) {
    app.add_system(movement);
}

pub fn movement(
    mut units: Query<(
        &Transform,
        &RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &mut Unit,
    )>,
) {
    for (transform, velocity, mut forces, mut unit) in units.iter_mut() {
        if let Some(target) = unit.target {
            let target = target.xy();
            let source = transform.translation.xy();

            let to_target = (target - source).extend(0.0);

            if to_target.length() < 2.0 {
                unit.target = None;
                continue;
            }

            let to_target = to_target.normalize();

            let desired_linvel: Vector<Real> =
                (to_target * SPEED).truncate().to_array().into();
            let current_linvel = velocity.linvel;

            let diff = desired_linvel - current_linvel;
            forces.force = diff * 1000.0;
        } else {
            forces.force = [0.0, 0.0].into();
        }
    }
}
