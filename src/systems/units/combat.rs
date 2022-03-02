use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Alignment, Unit, BASE_DAMAGE};

pub fn system(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut units: Query<&mut Unit>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(left, right) => {
                let left = left.entity();
                let right = right.entity();

                let left_alignment = unit_alignment(left, &units);
                let right_alignment = unit_alignment(right, &units);

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

fn unit_alignment(
    entity: Entity,
    units: &Query<&mut Unit>,
) -> Option<Alignment> {
    units.get(entity).map(|unit| unit.alignment).ok()
}

fn deal_damage(
    entity: Entity,
    units: &mut Query<&mut Unit>,
    commands: &mut Commands,
) {
    if let Ok(mut unit) = units.get_mut(entity) {
        unit.health -= BASE_DAMAGE;

        if unit.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
