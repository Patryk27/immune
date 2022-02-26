use std::str::FromStr;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::map::Map;

const MAP: &str = include_str!("./map.toml");

#[derive(Component, Debug)]
pub struct Node;

// TODO(pwy) rename to LymphNode? (not sure on the biological term ATM)
#[derive(Component, Clone, Debug)]
pub struct FactoryNode {
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

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let sprite_handle = assets.load("placeholder_circle.png");

    let map = Map::from_str(MAP).unwrap();

    for factory_node in map.factory_nodes {
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform::from_translation(factory_node.pos),
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(FactoryNode {
                time_to_spawn: 1.0,
                timer: 1.0,
                product: Some(FactoryProduct::Leukocyte(Leukocyte {
                    antigen: Antigen::Triangle,
                    body: Body::Hexagon,
                    kind: LeukocyteKind::Killer,
                })),
            });
    }

    for node in map.nodes {
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform::from_translation(node.pos),
                sprite: Sprite {
                    color: Color::BLUE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Node);
    }
}

pub fn brownian_motion_system(mut query: Query<(&Leukocyte, &mut Transform)>) {
    let mut rng = thread_rng();
    for (_, mut transform) in query.iter_mut() {
        transform.translation +=
            Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
    }
}

pub fn factory_node_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<(&mut FactoryNode, &Transform)>,
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

            // TODO: Do not load sprite every time, wtf?
            let sprite_handle = assets.load("placeholder_square.png");

            match product {
                FactoryProduct::Leukocyte(leukocyte) => {
                    let mut transform =
                        (*transform).with_scale(Vec3::splat(0.25));
                    transform.translation.z = 1.0;

                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: sprite_handle.clone(),
                            transform,
                            sprite: Sprite {
                                color: Color::GREEN,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(leukocyte.to_owned());
                }
            }
        }
    }
}
