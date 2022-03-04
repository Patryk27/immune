use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    Health, Unit, HEALTH_TO_SCALE, MAX_SPEED, MOVEMENT_SQUEEZE_FACTOR,
    MOVEMENT_STRETCH_FACTOR,
};
use crate::systems::bio::CellBody;

pub fn system(
    units: Query<(&Health, &Children, &RigidBodyVelocityComponent), With<Unit>>,
    mut child_sprites: Query<(&CellBody, &mut Transform)>,
) {
    for (health, children, velocity) in units.iter() {
        let direction = velocity.linvel.normalize();

        let up: Vector<Real> = [0.0, 1.0].into();
        let angle = direction.angle(&up);

        let angle = if direction.x < 0.0 {
            angle
        } else {
            std::f32::consts::PI - angle
        };

        for child in children.iter() {
            if let Ok((_, mut transform)) = child_sprites.get_mut(*child) {
                if velocity.linvel.magnitude() < 0.1 {
                    transform.scale =
                        Vec3::ONE * health.health * HEALTH_TO_SCALE;
                } else {
                    let stretch_x = remap(
                        velocity.linvel.magnitude(),
                        0.0,
                        MAX_SPEED,
                        1.0,
                        MOVEMENT_SQUEEZE_FACTOR,
                    );
                    let stretch_y = remap(
                        velocity.linvel.magnitude(),
                        0.0,
                        MAX_SPEED,
                        1.0,
                        MOVEMENT_STRETCH_FACTOR,
                    );

                    transform.rotation = Quat::from_rotation_z(angle);
                    transform.scale = Vec3::new(stretch_x, stretch_y, 1.0)
                        * health.health
                        * HEALTH_TO_SCALE;
                }
                break;
            }
        }
    }
}

fn remap(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
