use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Leukocyte, Pathogen};
use crate::systems::highlight::Selector;
use crate::systems::physics::PHYSICS_SCALE;
use crate::systems::units::{Alignment, Unit};
use crate::theme;

#[derive(Component)]
pub struct CellBody;

pub enum Cell<'a> {
    Leukocyte(&'a Leukocyte),
    Pathogen(&'a Pathogen),
}

impl<'a> Cell<'a> {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        pos: Vec2,
        vel: Vec2,
    ) {
        let mut entity = commands.spawn();

        let mut unit = Unit::default();

        entity
            .insert(Transform::from_translation(
                (pos * PHYSICS_SCALE).extend(theme::z_index::CELL),
            ))
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert_bundle(RigidBodyBundle {
                position: pos.to_array().into(),
                mass_properties: RigidBodyMassPropsComponent(
                    RigidBodyMassProps {
                        flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                        ..Default::default()
                    },
                ),
                velocity: RigidBodyVelocityComponent(RigidBodyVelocity {
                    linvel: vel.to_array().into(),
                    ..Default::default()
                }),
                damping: RigidBodyDampingComponent(RigidBodyDamping {
                    angular_damping: 1.0,
                    linear_damping: 1.0,
                }),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::ball(0.25)),
                material: ColliderMaterialComponent(ColliderMaterial {
                    friction: 0.1,
                    restitution: 0.5,
                    ..Default::default()
                }),
                flags: ColliderFlagsComponent(ColliderFlags {
                    active_events: ActiveEvents::CONTACT_EVENTS,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(CellFadeIn::default());

        match self {
            Cell::Leukocyte(cell) => {
                unit.alignment = Alignment::Player;
                entity.insert(**cell);
            }
            Cell::Pathogen(cell) => {
                unit.alignment = Alignment::Enemy;
                entity.insert(**cell);
            }
        }

        entity.insert(unit);

        let (body, color) = match self {
            Cell::Leukocyte(cell) => (cell.body, Leukocyte::color(0)),
            Cell::Pathogen(cell) => (cell.body, Pathogen::color(0)),
        };

        // Spawn cell's sprite
        entity.with_children(|entity| {
            entity
                .spawn()
                .insert(Transform::default())
                .insert(GlobalTransform::default())
                .insert(CellBody)
                .with_children(|entity| {
                    let texture = assets.load(body.asset_path());

                    entity
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                ..Default::default()
                            },
                            transform: Transform::default()
                                .with_scale(Vec3::splat(0.25)),
                            texture,
                            ..Default::default()
                        })
                        .insert(CellFadeIn::default());

                    // Spawn cell's antigens / antigen binders
                    match self {
                        Cell::Leukocyte(cell) => {
                            cell.binder.spawn(assets, entity, body, color)
                        }
                        Cell::Pathogen(cell) => {
                            cell.antigen.spawn(assets, entity, body, color)
                        }
                    }
                });
        });

        // Spawn cell's selector
        entity.with_children(|entity| {
            Selector::spawn(
                assets,
                entity,
                50.0,
                Color::rgba_u8(0, 220, 0, 50),
            );
        });
    }
}

// TODO(pwy) currently we set this for each of cell's sprites - I'd rather have
//           just one `FreshCell` tag per entity instead, for clarity
#[derive(Component, Debug, Default)]
pub struct CellFadeIn {
    pub tt: f32,
}