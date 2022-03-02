use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{MAX_SPEED, MOVEMENT_SQUEEZE_FACTOR, MOVEMENT_STRETCH_FACTOR};
use crate::systems::cell_node::CellBody;

pub fn system(
    units: Query<(&Children, &RigidBodyVelocityComponent)>,
    mut child_sprites: Query<(&CellBody, &mut Transform)>,
) {
    for (children, velocity) in units.iter() {
        if velocity.linvel.magnitude() < 0.1 {
            continue;
        }

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
                transform.scale = Vec3::new(stretch_x, stretch_y, 1.0);
            }
        }
    }
}

fn remap(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}