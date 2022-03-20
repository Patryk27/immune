use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::{Antigen, Leukocyte, Pathogen, Protein};
use crate::systems::input::{Collider, Selector};
use crate::systems::physics::PHYSICS_SCALE;
use crate::systems::units::combat::Weapon;
use crate::systems::units::{Alignment, DeathBehavior, Health, Unit};
use crate::theme;

#[derive(Component)]
pub struct CellBody;

pub enum Cell<'a> {
    Leukocyte(&'a Leukocyte),
    Pathogen(&'a Pathogen),
}

impl<'a> Cell<'a> {
    pub const SIZE: f32 = 0.125;

    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        pos: Vec2,
        vel: Vec2,
    ) {
        let mut rng = rand::thread_rng();
        let mut entity = commands.spawn();

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
                shape: ColliderShapeComponent(ColliderShape::ball(
                    Self::SIZE + Antigen::SIZE,
                )),
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
            .insert(Collider::Circle {
                radius: (Self::SIZE + Antigen::SIZE) * PHYSICS_SCALE,
            })
            .insert(CellFadeIn::default());

        match self {
            Cell::Leukocyte(cell) => {
                entity.insert(Weapon::AntigenBinder(cell.binder));
                entity.insert(Alignment::Player);
                entity.insert(Health::with_health(cell.props.hp as f32));
                entity.insert((*cell).to_owned());
            }
            Cell::Pathogen(cell) => {
                entity.insert(Weapon::Antigen(cell.antigen));
                entity.insert(Alignment::Enemy);
                entity.insert(Health::default());
                entity.insert((*cell).to_owned());
            }
        }

        entity
            .insert(Unit::default())
            .insert(DeathBehavior::Despawn);

        let (body, proteins, color) = match self {
            Cell::Leukocyte(cell) => {
                (cell.body, &cell.proteins[..], Leukocyte::color(0))
            }
            Cell::Pathogen(cell) => {
                (cell.body, [].as_slice(), Pathogen::color(0))
            }
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
                            transform: Transform::from_scale(Vec3::splat(
                                Self::SIZE,
                            )),
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

                    // Spawn cell's proteins
                    for protein in proteins {
                        let transform = Transform::from_scale(Vec3::splat(
                            Self::SIZE / 2.5,
                        ))
                        .with_translation(
                            vec3(
                                rng.gen_range(-1.0..1.0),
                                rng.gen_range(-1.0..1.0),
                                0.1,
                            ) * (Self::SIZE * PHYSICS_SCALE / 2.0),
                        );

                        let mut color = Protein::color();
                        color.set_a(0.8);

                        entity
                            .spawn()
                            .insert_bundle(SpriteBundle {
                                texture: assets.load(protein.asset_path()),
                                transform,
                                sprite: Sprite {
                                    color,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(CellFadeIn::default());
                    }
                });
        });

        // Spawn cell's selector
        entity.with_children(|entity| {
            Selector::spawn(
                assets,
                entity,
                2.4 * Self::SIZE * PHYSICS_SCALE,
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
