use bevy::prelude::*;
use bevy_rapier2d::physics::{ColliderBundle, RigidBodyBundle};
use bevy_rapier2d::prelude::{
    ColliderMaterial, ColliderMaterialComponent, ColliderShape,
    ColliderShapeComponent, RigidBodyType, RigidBodyTypeComponent,
};

use crate::level::{Level, LevelPoint};
use crate::systems::physics::PHYSICS_SCALE;
use crate::theme;

#[derive(Component, Clone, Copy, Debug)]
pub struct Wall {
    pub pos: LevelPoint,
}

impl Wall {
    pub const SIZE: f32 = 0.35;

    pub fn spawn(self, commands: &mut Commands, assets: &AssetServer) {
        let pos = Level::local_to_world(self.pos);

        let transform =
            Transform::from_translation(pos.extend(theme::z_index::WALL));

        // We have to spawn map scaled down to zero, since otherwise the fade-in
        // animation looks rather peculiar
        let transform = transform.with_scale(Vec3::splat(0.0));

        let mut entity = commands.spawn();

        entity
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert_bundle(RigidBodyBundle {
                position: (pos / PHYSICS_SCALE).to_array().into(),
                body_type: RigidBodyTypeComponent(RigidBodyType::Static),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::cuboid(
                    Self::SIZE / 2.0,
                    Self::SIZE / 2.0,
                )),
                material: ColliderMaterialComponent(ColliderMaterial {
                    friction: 0.1,
                    restitution: 0.5,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(self)
            .insert(WallFadeIn::default());

        // Spawn wall's sprite
        entity.with_children(|entity| {
            entity.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.75),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::splat(
                    Self::SIZE / 2.56 - 0.005,
                )),
                texture: assets.load("wall.png"),
                ..Default::default()
            });
        });
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WallFadeIn {
    pub progress: f32,
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WallFadeOut {
    pub progress: f32,
}
