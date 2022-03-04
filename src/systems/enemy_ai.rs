use std::collections::HashSet;

use bevy::prelude::*;

use super::bio::{LymphNode, Pathogen};
use super::units::{Alignment, Unit};

pub fn initialize(app: &mut App) {
    app.insert_resource(State::default())
        .insert_resource(EnemyAiEnabled(true))
        .add_system(attack_lymph_nodes)
        .add_system(detect_new_units)
        .add_system(detect_removed_units);
}

pub struct EnemyAiEnabled(pub bool);

#[derive(Default)]
pub struct State {
    pub units: HashSet<Entity>,
    pub combat_groups: Vec<CombatGroup>,
}

impl State {
    fn add_unit(&mut self, unit: Entity) {
        self.units.insert(unit);
    }

    fn remove_unit(&mut self, unit: Entity) {
        if self.units.remove(&unit) {
            return;
        }

        for combat_group in self.combat_groups.iter_mut() {
            if combat_group.units.remove(&unit) {
                return;
            }
        }
    }
}

pub struct CombatGroup {
    pub units: HashSet<Entity>,
}

fn attack_lymph_nodes(
    enabled: Res<EnemyAiEnabled>,
    state: Res<State>,
    mut pathogens: Query<(&mut Unit, &Pathogen)>,
    lymph_nodes: Query<(&Transform, &Alignment, &LymphNode)>,
) {
    if !enabled.0 {
        return;
    }

    let mut first_player_owned_lymph_node = None;
    for (transform, alignment, _) in lymph_nodes.iter() {
        if *alignment != Alignment::Enemy {
            first_player_owned_lymph_node = Some(transform.translation);
            break;
        }
    }

    if let Some(first_player_owned_lymph_node) = first_player_owned_lymph_node {
        for entity in state.units.iter() {
            if let Ok((mut unit, _)) = pathogens.get_mut(*entity) {
                unit.target = Some(first_player_owned_lymph_node.truncate());
            }
        }
    }
}

fn detect_new_units(
    mut state: ResMut<State>,
    query: Query<(Entity, &Unit, &Pathogen), Added<Pathogen>>,
) {
    for (entity, _, _) in query.iter() {
        state.add_unit(entity);
    }
}

fn detect_removed_units(
    mut state: ResMut<State>,
    removed_pathogens: RemovedComponents<Pathogen>,
) {
    for entity in removed_pathogens.iter() {
        state.remove_unit(entity);
    }
}
