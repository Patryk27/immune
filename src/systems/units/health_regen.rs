use bevy::prelude::*;

use super::Health;

pub fn system(time: Res<Time>, mut units: Query<&mut Health>) {
    for mut health in units.iter_mut() {
        health.health = (health.health
            + time.delta_seconds() * health.regen_rate)
            .clamp(0.0, health.max_health);
    }
}
