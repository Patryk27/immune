use std::iter;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use super::{AntigenBinder, Body, Leukocyte, Protein};
use crate::systems::highlight::Selector;
use crate::systems::physics::PHYSICS_SCALE;
use crate::theme;

#[derive(Component, Clone, Debug)]
pub struct LymphNode {
    pub lhs: Option<LymphNodeInput>,
    pub rhs: Option<LymphNodeInput>,
    pub output: Option<LymphNodeOutput>,
    pub function: LymphNodeFunction,
    pub state: LymphNodeState,
    pub production_tt: f32,
}

impl LymphNode {
    pub const PRODUCTION_DURATION: f32 = 1.5;

    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        assets: &AssetServer,
        at: Vec2,
    ) {
        let transform = Transform::from_translation(
            (at * PHYSICS_SCALE).extend(theme::z_index::LYMPH_NODE),
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
                    color: Color::rgb_u8(222, 0, 222),
                    ..Default::default()
                },
                texture: assets.load("lymph-node.png"),
                ..Default::default()
            });
        });

        // Spawn lymph node's warning
        entity.with_children(|entity| {
            LymphNodeWarning::spawn(entity);
        });

        // Spawn lymph node's progress bar
        entity.with_children(|entity| {
            LymphNodeProgressBar::spawn(meshes, materials, entity);
        });

        // Spawn lymph node's selector
        entity.with_children(|entity| {
            Selector::spawn(
                assets,
                entity,
                140.0,
                Color::rgba_u8(242, 185, 56, 50),
            );
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeFunction {
    Producer,
    Provider,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LymphNodeState {
    pub paused: bool,
    pub awaiting_resources: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeInput {
    Body(Body),
    Binder(AntigenBinder),
    Protein(Protein),
    External(Option<Entity>),
}

impl LymphNodeInput {
    pub fn variants() -> impl Iterator<Item = Self> {
        let bodies = Body::variants().map(Self::Body);
        let binders = AntigenBinder::variants().map(Self::Binder);
        let proteins = Protein::variants().map(Self::Protein);

        bodies
            .chain(binders)
            .chain(proteins)
            .chain(iter::once(Self::External(None)))
    }

    pub fn asset_path(&self) -> Option<&'static str> {
        match self {
            Self::Body(body) => Some(body.asset_path()),
            Self::Binder(binder) => Some(binder.asset_path()),
            Self::Protein(protein) => Some(protein.asset_path()),
            Self::External(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LymphNodeOutput {
    Leukocyte(Leukocyte),
}

#[derive(Component, Clone, Debug)]
pub struct LymphNodeWarning {
    pub asset_path: Option<&'static str>,
    pub dirty: bool,
    pub tt: f32,
}

impl LymphNodeWarning {
    pub fn spawn(entity: &mut ChildBuilder) {
        let transform = Transform::default()
            .with_translation(vec3(
                0.0,
                25.0,
                theme::z_index::LYMPH_NODE_COMPILATION_WARNING
                    - theme::z_index::LYMPH_NODE,
            ))
            .with_scale(vec3(0.8, 0.8, 1.0));

        entity
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(255, 190, 17),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            })
            .insert(Self {
                asset_path: None,
                dirty: true,
                tt: 0.0,
            });
    }

    pub fn set(&mut self, new_asset_path: Option<&'static str>) {
        if new_asset_path != self.asset_path {
            self.dirty = true;
            self.asset_path = new_asset_path;
        }
    }
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
                        size: vec2(220.0, 15.0),
                        ..Default::default()
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..Default::default()
            })
            .insert(Self);
    }
}

#[derive(Component, Debug)]
pub struct LymphNodeConnection {
    pub source: Vec2,
    pub source_entity: Entity,
    pub target: Vec2,
    pub target_entity: Entity,
    pub points: Vec<LymphNodeConnectionPoint>,
    pub tint_r: f32,
    pub tint_g: f32,
    pub tint_b: f32,
    pub tt: f32,
}

impl LymphNodeConnection {
    pub fn new(
        source: Vec2,
        source_entity: Entity,
        target: Vec2,
        target_entity: Entity,
    ) -> Self {
        const SEGMENT_LEN: f32 = 4.0;

        let mut rng = rand::thread_rng();
        let points_count = (source.distance(target) / SEGMENT_LEN) as i32;

        let points = (0..points_count).map(|idx| {
            let mut pos = source
                + (target - source) / (points_count as f32) * (idx as f32);

            pos.x += rng.gen_range(-3.0..3.0);
            pos.y += rng.gen_range(-3.0..3.0);

            LymphNodeConnectionPoint {
                pos,
                vel: Default::default(),
            }
        });

        Self {
            source,
            source_entity,
            target,
            target_entity,
            points: points.collect(),
            tint_r: rng.gen_range(-0.02..0.1),
            tint_g: rng.gen_range(-0.2..0.2),
            tint_b: rng.gen_range(-0.2..0.2),
            tt: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct LymphNodeConnectionPoint {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(Component, Default)]
pub struct DeadLymphNodeConnection {
    pub tt: f32,
    pub in_progress: bool,
}
