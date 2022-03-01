use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use super::{AntigenBinder, Body, Leukocyte, Protein};
use crate::compiling::CompilationWarning;
use crate::systems::physics::PHYSICS_SCALE;
use crate::z_index;

#[derive(Component, Clone, Debug)]
pub struct LymphNode {
    pub lhs: Option<LymphNodeInput>,
    pub rhs: Option<LymphNodeInput>,
    pub output: Option<LymphNodeOutput>,
    pub production_tt: f32,
    pub production_duration: f32,
}

impl LymphNode {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        assets: &AssetServer,
        at: Vec2,
    ) {
        let transform = Transform::from_translation(
            (at * PHYSICS_SCALE).extend(z_index::LYMPH_NODE),
        )
        .with_scale(Vec3::splat(0.5));

        let mut entity = commands.spawn();

        entity
            .insert(transform)
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(self.to_owned());

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
            CompilationWarning::spawn(assets, entity);
        });

        // Spawn lymph node's progress bar
        entity.with_children(|entity| {
            LymphNodeProgressBar::spawn(meshes, materials, entity);
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

#[derive(Component, Debug)]
pub struct LymphNodeProgressBar;

impl LymphNodeProgressBar {
    pub fn spawn(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        entity: &mut ChildBuilder,
    ) {
        entity
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: vec2(250.0, 15.0),
                        ..Default::default()
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..Default::default()
            })
            .insert(Self);
    }
}
