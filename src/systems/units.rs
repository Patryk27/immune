use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

const SPEED: f32 = 500.0;

#[derive(Component, Default)]
pub struct Unit {
    // TODO(dzejkop): Should be enum, target can be unit, etc.
    pub target: Option<Vec3>,
}

pub fn initialize(app: &mut App) {
    app.add_system(movement);
}

pub fn movement(
    time: Res<Time>,
    mut units: Query<(&mut Transform, &mut Unit)>,
) {
    let speed = SPEED * time.delta_seconds();

    for (mut transform, unit) in units.iter_mut() {
        if let Some(target) = unit.target {
            let target = target.xy();
            let source = transform.translation.xy();

            let to_target = (target - source).extend(0.0);

            if to_target.length() < 2.0 {
                continue;
            }

            let to_target = to_target.normalize();

            transform.translation += to_target * speed;
        }
    }
}
