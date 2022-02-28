use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Leukocyte, Pathogen};
use crate::systems::highlight::Highlight;
use crate::systems::units::Unit;

pub enum Cell<'a> {
    Leukocyte(&'a Leukocyte),
    Pathogen(&'a Pathogen),
}

impl<'a> Cell<'a> {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        at: Vec2,
    ) {
        let mut entity = commands.spawn();

        entity
            .insert(Transform::from_translation(at.extend(1.0)))
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert_bundle(RigidBodyBundle {
                position: at.to_array().into(),
                damping: RigidBodyDampingComponent(RigidBodyDamping {
                    angular_damping: 1.0,
                    linear_damping: 1.0,
                }),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::ball(0.25)),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .insert(Unit::default());

        match self {
            Cell::Leukocyte(cell) => {
                entity.insert(**cell);
            }
            Cell::Pathogen(cell) => {
                entity.insert(**cell);
            }
        }

        let (body, color) = match self {
            Cell::Leukocyte(cell) => (cell.body, Color::WHITE),
            Cell::Pathogen(cell) => (cell.body, Color::RED),
        };

        // Spawn cell's sprite
        entity.with_children(|entity| {
            let texture = assets.load(body.asset_path());

            let sprite = SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                transform: Transform::default().with_scale(Vec3::splat(0.25)),
                texture,
                ..Default::default()
            };

            entity.spawn_bundle(sprite);
        });

        // Spawn cell's antigens / antigen binders
        entity.with_children(|entity| match self {
            Cell::Leukocyte(cell) => cell.binder.spawn(assets, entity, body),
            Cell::Pathogen(cell) => cell.antigen.spawn(assets, entity, body),
        });

        // Spawn hidden selection highlight
        entity.with_children(|entity| {
            let texture = assets.load("selector.png");
            let color = Color::rgba_u8(0, 220, 0, 50);
            let size = 50.0; // TODO(pry): this info should be within unit struct
            let arrows = vec![
                (false, false, -1.0, 1.0),
                (true, false, 1.0, 1.0),
                (false, true, -1.0, -1.0),
                (true, true, 1.0, -1.0),
            ];

            for (flip_x, flip_y, mul_x, mul_y) in arrows {
                let transform =
                    Transform::from_xyz(size * mul_x, size * mul_y, 0.0);

                entity
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color,
                            flip_x,
                            flip_y,
                            ..Default::default()
                        },
                        texture: texture.clone(),
                        transform,
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(Highlight);
            }
        });
    }
}
