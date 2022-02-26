use std::str::FromStr;

use bevy::prelude::*;

use super::units::Unit;
use crate::map::Map;

const MAP: &str = include_str!("./map.toml");

#[derive(Component, Debug)]
pub struct Node;

// TODO(pwy) rename to LymphNode? (not sure on the biological term ATM)
#[derive(Component, Clone, Debug)]
pub struct Factory {
    pub time_to_spawn: f32,
    pub timer: f32,
    pub product: Option<FactoryProduct>,
}

#[derive(Clone, Debug)]
pub enum FactoryProduct {
    Leukocyte(Leukocyte),
}

#[derive(Component, Clone, Debug)]
pub struct Leukocyte {
    pub antigen: Antigen,
    pub body: Body,
    pub kind: LeukocyteKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeukocyteKind {
    // Cager, TODO(pwy) post-MVP
    Killer,
}

#[derive(Component, Debug)]
pub struct Pathogen {
    pub antigen: Antigen,
    pub shape: Body,
    pub kind: PathogenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathogenKind {
    Virus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Antigen {
    Rectangle,
    Semicircle,
    Triangle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Body {
    Circle,
    Hexagon,
}

// ---

#[derive(Bundle)]
pub struct FactoryBundle {
    pub factory: Factory,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl FactoryBundle {
    pub fn new(
        assets: &AssetServer,
        factory: Factory,
        translation: Vec2,
    ) -> Self {
        let transform = Transform::from_translation(translation.extend(0.9))
            .with_scale(Vec3::splat(0.5));

        let texture = assets.load("body-circle.png");

        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(195, 160, 229),
                ..Default::default()
            },
            transform,
            texture,
            ..Default::default()
        };

        Self { factory, sprite }
    }
}

// ---

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let map = Map::from_str(MAP).unwrap();

    for (idx, map_factory) in map.factory_nodes.iter().enumerate() {
        let factory = {
            // TODO this should be set by the user
            let body = if idx % 2 == 0 {
                Body::Hexagon
            } else {
                Body::Circle
            };

            Factory {
                time_to_spawn: 1.0,
                timer: 1.0,
                product: Some(FactoryProduct::Leukocyte(Leukocyte {
                    antigen: Antigen::Triangle,
                    body,
                    kind: LeukocyteKind::Killer,
                })),
            }
        };

        commands.spawn_bundle(FactoryBundle::new(
            &assets,
            factory,
            map_factory.pos,
        ));
    }
}

pub fn factory_node_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<(&mut Factory, &Transform)>,
) {
    for (mut factory_node, transform) in &mut query.iter_mut() {
        let factory_node = &mut *factory_node;

        let product = if let Some(product) = &factory_node.product {
            product
        } else {
            continue;
        };

        factory_node.timer -= time.delta_seconds();

        if factory_node.timer <= 0.0 {
            factory_node.timer = factory_node.time_to_spawn;

            match product {
                FactoryProduct::Leukocyte(leukocyte) => {
                    let transform = Transform::from_translation(
                        transform.translation.truncate().extend(1.0),
                    );

                    commands
                        .spawn()
                        .insert(transform)
                        .insert(GlobalTransform::default())
                        .insert(Visibility::default())
                        .insert(leukocyte.to_owned())
                        .insert(Unit::default())
                        .with_children(|entity| {
                            let texture = assets.load(match leukocyte.body {
                                Body::Circle => "body-circle.png",
                                Body::Hexagon => "body-hexagon.png",
                            });

                            let sprite = SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgb_u8(255, 255, 255),
                                    ..Default::default()
                                },
                                transform: Transform::default()
                                    .with_scale(Vec3::splat(0.25)),
                                texture,
                                ..Default::default()
                            };

                            entity.spawn_bundle(sprite);
                        });
                }
            }
        }
    }
}
