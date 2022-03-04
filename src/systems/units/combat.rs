use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Alignment, DeathBehavior, Health, BASE_DAMAGE};

pub fn system(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut units: Query<(&mut Alignment, &DeathBehavior, &mut Health)>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(left, right) => {
                let left = left.entity();
                let right = right.entity();

                let left_alignment = alignment_of(left, &units);
                let right_alignment = alignment_of(right, &units);

                match (left_alignment, right_alignment) {
                    (Some(left_alignment), Some(right_alignment)) => {
                        if left_alignment != right_alignment {
                            deal_damage(left, &mut units, &mut commands);
                            deal_damage(right, &mut units, &mut commands);
                        }
                    }
                    _ => {}
                }
            }
            ContactEvent::Stopped(_, _) => (),
        }
    }
}

fn alignment_of(
    entity: Entity,
    units: &Query<(&mut Alignment, &DeathBehavior, &mut Health)>,
) -> Option<Alignment> {
    units.get(entity).map(|(alignment, _, _)| *alignment).ok()
}

fn deal_damage(
    entity: Entity,
    units: &mut Query<(&mut Alignment, &DeathBehavior, &mut Health)>,
    commands: &mut Commands,
) {
    if let Ok((mut alignment, death_behavior, mut health)) =
        units.get_mut(entity)
    {
        health.health -= BASE_DAMAGE;

        if health.health <= 0.0 {
            match death_behavior {
                DeathBehavior::Die => {
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
