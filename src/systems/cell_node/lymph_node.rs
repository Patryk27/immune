use bevy::prelude::*;

use super::{AntigenBinder, Body, Leukocyte, Protein};
use crate::compiling::{CompilationWarning, NeedsRecompiling};

#[derive(Component, Clone, Debug)]
pub struct LymphNode {
    pub time_to_spawn: f32,
    pub timer: f32,
    pub lhs: Option<LymphNodeInput>,
    pub rhs: Option<LymphNodeInput>,
    pub output: Option<LymphNodeOutput>,
}

impl LymphNode {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        at: Vec2,
    ) {
        let transform = Transform::from_translation(at.extend(0.9))
            .with_scale(Vec3::splat(0.5));

        let mut entity = commands.spawn();

        entity
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(self.to_owned())
            .insert(NeedsRecompiling);

        // Spawn lymph node's sprite
        entity.with_children(|entity| {
            entity.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(195, 160, 229),
                    ..Default::default()
                },
                texture: assets.load("body.circle.png"),
                ..Default::default()
            });
        });

        // Spawn lymph node's compilation warning
        entity.with_children(|entity| {
            entity
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb_u8(255, 0, 0),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    texture: assets.load("warning.png"),
                    ..Default::default()
                })
                .insert(CompilationWarning);
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeInput {
    Body(Body),
    Binder(AntigenBinder),
    Protein(Protein),
    External(Entity),
}

impl LymphNodeInput {
    pub fn variants() -> impl Iterator<Item = Self> {
        let bodies = Body::variants().map(Self::Body);
        let binders = AntigenBinder::variants().map(Self::Binder);
        let proteins = Protein::variants().map(Self::Protein);

        bodies.chain(binders).chain(proteins)
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Body(body) => body.asset_path(),
            Self::Binder(binder) => binder.asset_path(),
            Self::Protein(protein) => protein.asset_path(),
            Self::External(_) => Body::Circle.asset_path(), // TODO(pwy) needs its own icon
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LymphNodeOutput {
    Leukocyte(Leukocyte),
}
