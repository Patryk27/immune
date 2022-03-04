use std::cmp::Ordering;
use std::collections::HashSet;

use bevy::prelude::*;

use super::bio::{LymphNode, Pathogen};
use super::units::{Alignment, Unit};

// Max size of a combat group
const COMBAT_GROUP_SIZE: usize = 100;

// Units will seek groups within this distance
const COMBAT_GROUP_DISTANCE_THRESHOLD: f32 = 300.0;

// How many seconds will an undermanned group wait before moving out
const MAX_WAIT_TIME_BEFORE_ATTACKING: f32 = 20.0;

pub fn initialize(app: &mut App) {
    app.insert_resource(State::default())
        .insert_resource(EnemyAiEnabled(true))
        .add_system(attack_lymph_nodes)
        .add_system(track_combat_group_center)
        .add_system(track_unit_alignment)
        .add_system(detect_removed_units)
        .add_system(detect_new_units);
}

pub struct EnemyAiEnabled(pub bool);

#[derive(Default)]
pub struct State {
    pub combat_groups: Vec<CombatGroup>,
}

impl State {
    fn add_unit(&mut self, time: f32, unit: Entity, pos: Vec2) {
        if self.combat_groups.iter().any(|group| group.contains(&unit)) {
            return;
        }

        let mut closest_group = None;
        let mut closest_distance = None;

        for (idx, combat_group) in self.combat_groups.iter().enumerate() {
            let distance = combat_group.center.distance(pos);

            if distance > COMBAT_GROUP_DISTANCE_THRESHOLD {
                continue;
            }

            if combat_group.len() > COMBAT_GROUP_SIZE {
                continue;
            }

            if closest_group.is_none() {
                closest_group = Some(idx);
                closest_distance = Some(distance);
            } else {
                if distance < closest_distance.unwrap() {
                    closest_group = Some(idx);
                    closest_distance = Some(distance);
                }
            }
        }

        if closest_group.is_none() {
            let mut units = HashSet::new();
            units.insert(unit);
            self.combat_groups.push(CombatGroup {
                creation_time: time,
                units,
                center: pos,
            });
        } else {
            let idx = closest_group.unwrap();
            self.combat_groups[idx].add_unit(unit);
        }
    }

    fn remove_unit(&mut self, unit: Entity) {
        for combat_group in self.combat_groups.iter_mut() {
            if combat_group.units.remove(&unit) {
                return;
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct CombatGroup {
    pub creation_time: f32,
    pub units: HashSet<Entity>,
    pub center: Vec2,
}

impl CombatGroup {
    fn len(&self) -> usize {
        self.units.len()
    }

    fn add_unit(&mut self, unit: Entity) {
        self.units.insert(unit);
    }

    fn contains(&self, entity: &Entity) -> bool {
        self.units.contains(entity)
    }
}

fn attack_lymph_nodes(
    time: Res<Time>,
    enabled: Res<EnemyAiEnabled>,
    state: Res<State>,
    mut pathogens: Query<(&mut Unit, &Alignment), Without<LymphNode>>,
    lymph_nodes: Query<(&Transform, &Alignment, &LymphNode), With<LymphNode>>,
) {
    if !enabled.0 {
        return;
    }

    let player_owned_lymph_nodes: Vec<Vec2> = lymph_nodes
        .iter()
        .filter(|(_, alignment, _)| alignment.is_player())
        .map(|(transform, _, _)| transform.translation.truncate())
        .collect();

    for combat_group in state.combat_groups.iter() {
        let combat_group_age =
            time.seconds_since_startup() as f32 - combat_group.creation_time;

        if combat_group_age < MAX_WAIT_TIME_BEFORE_ATTACKING
            && combat_group.len() < COMBAT_GROUP_SIZE
        {
            for unit in combat_group.units.iter() {
                if let Ok((mut unit, _)) = pathogens.get_mut(*unit) {
                    unit.target = Some(combat_group.center);
                }
            }
        } else {
            let closest_lymph_node = player_owned_lymph_nodes
                .iter()
                .map(|pos| (pos, pos.distance(combat_group.center)))
                .min_by(|(_, lhs), (_, rhs)| {
                    lhs.partial_cmp(rhs).unwrap_or(Ordering::Greater)
                });

            if let Some((pos, _)) = closest_lymph_node {
                for unit in combat_group.units.iter() {
                    if let Ok((mut unit, _)) = pathogens.get_mut(*unit) {
                        unit.target = Some(*pos);
                    }
                }
            }
        }
    }
}

fn track_combat_group_center(
    mut state: ResMut<State>,
    query: Query<(&Unit, &Transform)>,
) {
    for combat_group in state.combat_groups.iter_mut() {
        let mut center = Vec2::ZERO;

        for unit in combat_group.units.iter() {
            if let Ok((_, transform)) = query.get(*unit) {
                center += transform.translation.truncate();
            }
        }

        combat_group.center = center / combat_group.len() as f32;
    }
}

fn track_unit_alignment(
    time: Res<Time>,
    mut state: ResMut<State>,
    query: Query<(Entity, &Unit, &Transform, &Alignment), Changed<Alignment>>,
) {
    for (entity, _, transform, alignment) in query.iter() {
        if alignment.is_enemy() {
            state.add_unit(
                time.seconds_since_startup() as f32,
                entity,
                transform.translation.truncate(),
            );
        } else {
            state.remove_unit(entity);
        }
    }
}

fn detect_new_units(
    time: Res<Time>,
    mut state: ResMut<State>,
    query: Query<(Entity, &Unit, &Transform, &Pathogen), Added<Pathogen>>,
) {
    for (entity, _, transform, _) in query.iter() {
        state.add_unit(
            time.seconds_since_startup() as f32,
            entity,
            transform.translation.truncate(),
        );
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
