use bevy::prelude::*;

use super::bio::Pathogen;
use super::units::Unit;

pub fn initialize(app: &mut App) {
    app.add_system(attack_lymph_nodes);
}

fn attack_lymph_nodes(mut pathogens: Query<(&mut Unit, &Pathogen)>) {
    for (unit, _) in pathogens.iter_mut() {
        if unit.target.is_none() {
            // TODO: Issue commands to units
        }
    }
}
