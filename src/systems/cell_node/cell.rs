use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Leukocyte, Pathogen};
use crate::systems::highlight::Highlight;
use crate::systems::physics::PHYSICS_SCALE;
use crate::systems::units::Unit;
use crate::z_index;

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

        entity
            .insert(Transform::from_translation(
                (pos * PHYSICS_SCALE).extend(z_index::CELL),
            ))
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert_bundle(RigidBodyBundle {
                position: pos.to_array().into(),
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
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(Unit::default())
            .insert(CellFadeIn::default());

        match self {
            Cell::Leukocyte(cell) => {
                entity.insert(**cell);
            }
            Cell::Pathogen(cell) => {
                entity.insert(**cell);
            }
        }

        let (body, color) = match self {
            Cell::Leukocyte(cell) => {
                (cell.body, Color::rgba_u8(255, 255, 255, 0))
            }
            Cell::Pathogen(cell) => (cell.body, Color::rgba_u8(255, 0, 0, 0)),
        };

        // Spawn cell's sprite
        entity.with_children(|entity| {
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

// TODO(pwy) currently we set this for each of cell's sprites - I'd rather have
//           just one `FreshCell` tag per entity instead, for clarity
#[derive(Component, Debug, Default)]
pub struct CellFadeIn {
    pub tt: f32,
}
