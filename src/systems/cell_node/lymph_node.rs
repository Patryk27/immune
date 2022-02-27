use bevy::prelude::*;

use super::{AntigenBinder, Body, Leukocyte, Protein};
use crate::compiler::Compiler;

#[derive(Component, Clone, Debug)]
pub struct LymphNode {
    pub time_to_spawn: f32,
    pub timer: f32,
    pub lhs: Option<LymphNodeInput>,
    pub rhs: Option<LymphNodeInput>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeInput {
    Binder(AntigenBinder),
    Body(Body),
    Protein(Protein),
    External(Entity),
}

#[derive(Clone, Debug)]
pub enum LymphNodeOutput {
    Leukocyte(Leukocyte),
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

        commands
            .spawn()
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(self.to_owned())
            .with_children(|entity| {
                let texture = assets.load("body.circle.png");

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

    pub fn output(&self, compiler: &Compiler) -> Option<LymphNodeOutput> {
        compiler.compile(self.lhs, self.rhs)
    }
}

impl LymphNodeInput {
    pub fn variants() -> impl Iterator<Item = Self> {
        let binders = AntigenBinder::variants().map(Self::Binder);
        let bodies = Body::variants().map(Self::Body);
        let proteins = Protein::variants().map(Self::Protein);

        binders.chain(bodies).chain(proteins)
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Binder(binder) => binder.asset_path(),
            Self::Body(body) => body.asset_path(),
            Self::Protein(protein) => protein.asset_path(),
            Self::External(_) => Body::Circle.asset_path(), // TODO(pwy) needs its own icon
        }
    }
}
