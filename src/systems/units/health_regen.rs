use bevy::prelude::*;

use super::Unit;

pub fn system(time: Res<Time>, mut units: Query<&mut Unit>) {
    for mut unit in units.iter_mut() {
        unit.health = (unit.health + time.delta_seconds() * unit.regen_rate)
            .clamp(0.0, unit.max_health);
    }
}
