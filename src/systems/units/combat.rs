use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Alignment, DeathBehavior, Health, BASE_DAMAGE};
use crate::systems::bio::{Antigen, AntigenBinder};

const MATCHING_WEAPON_BONUS: f32 = 0.3;

#[derive(Clone, Copy, Component)]
pub enum Weapon {
    Antigen(Antigen),
    AntigenBinder(AntigenBinder),
    None,
}

pub fn system(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut units: Query<(&mut Alignment, &Weapon, &DeathBehavior, &mut Health)>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(left, right) => {
                let left = left.entity();
                let right = right.entity();

                match (
                    weapon_and_alignment_of(left, &units),
                    weapon_and_alignment_of(right, &units),
                ) {
                    (
                        Some((left_weapon, left_alignment)),
                        Some((right_weapon, right_alignment)),
                    ) => {
                        if left_alignment != right_alignment {
                            let damage_against_left =
                                calculate_damage(right_weapon, left_weapon);
                            let damage_against_right =
                                calculate_damage(left_weapon, right_weapon);

                            deal_damage(
                                left,
                                damage_against_left,
                                &mut units,
                                &mut commands,
                            );
                            deal_damage(
                                right,
                                damage_against_right,
                                &mut units,
                                &mut commands,
                            );
                        }
                    }
                    _ => {}
                }
            }
            ContactEvent::Stopped(_, _) => (),
        }
    }
}

fn weapon_and_alignment_of(
    entity: Entity,
    units: &Query<(&mut Alignment, &Weapon, &DeathBehavior, &mut Health)>,
) -> Option<(Weapon, Alignment)> {
    units
        .get(entity)
        .map(|(alignment, weapon, _, _)| (*weapon, *alignment))
        .ok()
}

fn deal_damage(
    entity: Entity,
    damage: f32,
    units: &mut Query<(&mut Alignment, &Weapon, &DeathBehavior, &mut Health)>,
    commands: &mut Commands,
) {
    if let Ok((mut alignment, _, death_behavior, mut health)) =
        units.get_mut(entity)
    {
        health.health -= damage;

        if health.health <= 0.0 {
            match death_behavior {
                DeathBehavior::Despawn => {
                    commands.entity(entity).despawn_recursive()
                }
                DeathBehavior::SwitchSides => {
                    alignment.flip();
                    health.reset();
                }
            }
        }
    }
}

fn calculate_damage(left: Weapon, right: Weapon) -> f32 {
    match (left, right) {
        (
            Weapon::Antigen(left),
            Weapon::AntigenBinder(AntigenBinder(right)),
        ) if left == right => {
            // Matching pathogen's antigen deals reduced damage against correct binder
            BASE_DAMAGE * (1.0 - MATCHING_WEAPON_BONUS)
        }
        (
            Weapon::AntigenBinder(AntigenBinder(left)),
            Weapon::Antigen(right),
        ) if left == right => {
            // Matching leukocyte's antigen binder deals increased damage against correct antigen
            BASE_DAMAGE * (1.0 + MATCHING_WEAPON_BONUS)
        }
        (Weapon::None, _) => {
            // No weapon (lymph node) deals reduced damage
            BASE_DAMAGE * (1.0 - MATCHING_WEAPON_BONUS)
        }
        _ => BASE_DAMAGE,
    }
}
