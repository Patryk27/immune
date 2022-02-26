use std::f32::consts::PI;
use std::str::FromStr;

use bevy::prelude::*;

use super::units::Unit;
use crate::map::Map;

const MAP: &str = include_str!("./map.toml");

// TODO(pwy) rename to LymphNode? (not sure on the biological term ATM)
#[derive(Component, Copy, Clone, Debug)]
pub struct Factory {
    pub time_to_spawn: f32,
    pub timer: f32,
    pub product: Option<FactoryProduct>,
}

impl Factory {
    pub fn spawn(
        self,
        commands: &mut Commands,
        assets: &AssetServer,
        at: Vec2,
    ) {
        let transform = Transform::from_translation(at.extend(0.9))
            .with_scale(Vec3::splat(0.5));

        commands
            .spawn()
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(self)
            .with_children(|entity| {
                let texture = assets.load("body-circle.png");

                entity.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb_u8(195, 160, 229),
                        ..Default::default()
                    },
                    texture,
                    ..Default::default()
                });
            });
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FactoryProduct {
    Leukocyte(Leukocyte),
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Leukocyte {
    pub antigen: Antigen,
    pub body: Body,
    pub kind: LeukocyteKind,
}

impl Leukocyte {
    pub fn spawn(
        self,
        commands: &mut Commands,
        assets: &AssetServer,
        at: Vec2,
    ) {
        let transform = Transform::from_translation(at.extend(1.0));

        commands
            .spawn()
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(self)
            .insert(Unit::default())
            .with_children(|entity| {
                let texture = assets.load(match self.body {
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
            })
            .with_children(|entity| {
                match (self.body, self.antigen) {
                    (Body::Circle, Antigen::Rectangle) => {
                        // TODO
                    }

                    (Body::Circle, Antigen::Semicircle) => {
                        // TODO
                    }

                    (Body::Circle, Antigen::Triangle) => {
                        const SIDES: u8 = 3;

                        let texture = assets.load("antigen-triangle.png");

                        for side in 0..SIDES {
                            let sprite = Sprite {
                                color: Color::rgba_u8(255, 255, 255, 100),
                                ..Default::default()
                            };

                            let transform_rot = Transform::from_rotation(
                                Quat::from_rotation_z(
                                    (side as f32) * 2.0 * PI / (SIDES as f32),
                                ),
                            );

                            let transform_pos = Transform::from_translation(
                                Vec3::new(0.0, 40.0, 0.0),
                            );

                            let transform = (transform_rot * transform_pos)
                                .with_scale(Vec3::splat(0.1));

                            let sprite = SpriteBundle {
                                sprite,
                                transform,
                                texture: texture.clone(),
                                ..Default::default()
                            };

                            entity.spawn_bundle(sprite);
                        }
                    }

                    (Body::Hexagon, Antigen::Rectangle) => {
                        // TODO
                    }

                    (Body::Hexagon, Antigen::Semicircle) => {
                        // TODO
                    }

                    (Body::Hexagon, Antigen::Triangle) => {
                        //
                    }
                }
            });
    }
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

// TODO(pwy/post-mvp) allow to configure number of sides
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

        factory.spawn(&mut commands, &assets, map_factory.pos);
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
                    leukocyte.spawn(
                        &mut commands,
                        &assets,
                        transform.translation.truncate(),
                    );
                }
            }
        }
    }
}
