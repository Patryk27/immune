use bevy::prelude::*;
use bevy_rapier2d::physics::{ColliderBundle, RigidBodyBundle};
use bevy_rapier2d::prelude::{
    ColliderMaterial, ColliderMaterialComponent, ColliderShape,
    ColliderShapeComponent, RigidBodyType, RigidBodyTypeComponent,
};

use crate::systems::physics::PHYSICS_SCALE;
use crate::theme;

#[derive(Component, Clone, Copy, Debug)]
pub struct Wall;

impl Wall {
    pub const SIZE: f32 = 0.4;

    pub fn spawn(commands: &mut Commands, assets: &AssetServer, pos: Vec2) {
        let transform = Transform::from_translation(
            (pos * PHYSICS_SCALE).extend(theme::z_index::WALL),
        )
        .with_scale(Vec3::splat(Self::SIZE / 2.0 - 0.05));

        let mut entity = commands.spawn();

        entity
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.75),
                    ..Default::default()
                },
                transform,
                texture: assets.load("wall.png"),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: pos.to_array().into(),
                body_type: RigidBodyTypeComponent(RigidBodyType::Static),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::cuboid(
                    Self::SIZE,
                    Self::SIZE,
                )),
                material: ColliderMaterialComponent(ColliderMaterial {
                    friction: 0.1,
                    restitution: 0.5,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(Self);
    }
}
