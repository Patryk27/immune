use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::physics::{
    ColliderBundle, ColliderPositionSync, RigidBodyBundle,
};
use bevy_rapier2d::prelude::{
    ColliderMaterial, ColliderMaterialComponent, ColliderShape,
    ColliderShapeComponent, RigidBodyType, RigidBodyTypeComponent,
};
use itertools::Itertools;
use rand::Rng;

use super::{AntigenBinder, Body, Leukocyte, Pathogen, Protein};
use crate::compiling::CompilationWarning;
use crate::systems::input::{Collider, Selector};
use crate::systems::physics::PHYSICS_SCALE;
use crate::systems::units::combat::Weapon;
use crate::systems::units::{Alignment, DeathBehavior, Health};
use crate::theme;

#[derive(Component, Clone, Debug)]
pub struct LymphNode {
    pub resource: Option<LymphNodeResource>,
    pub target: LymphNodeTarget,
    pub product: Option<LymphNodeProduct>,
    pub parent: Option<Entity>,
    pub warning: Option<CompilationWarning>,
    pub state: LymphNodeState,
    pub production_tt: f32,
}

impl LymphNode {
    pub const SIZE: f32 = 0.45;
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
            .insert_bundle(RigidBodyBundle {
                position: at.to_array().into(),
                body_type: RigidBodyTypeComponent(RigidBodyType::Static),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::ball(Self::SIZE)),
                material: ColliderMaterialComponent(ColliderMaterial {
                    friction: 0.1,
                    restitution: 0.5,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .insert(Collider::Circle {
                radius: Self::SIZE * PHYSICS_SCALE,
            })
            .insert(Health::lymph_node())
            .insert(Alignment::Player)
            .insert(DeathBehavior::SwitchSides)
            .insert(Weapon::None)
            .insert(self.to_owned());

        // Spawn lymph node's sprite
        entity.with_children(|entity| {
            entity.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(222, 0, 222),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::splat(Self::SIZE)),
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
                1.5 * Self::SIZE * PHYSICS_SCALE,
                Color::rgba_u8(242, 185, 56, 50),
            );
        });
    }

    pub fn is_spawner(&self) -> bool {
        matches!(self.target, LymphNodeTarget::Outside)
            && matches!(
                self.product,
                Some(
                    LymphNodeProduct::Leukocyte(_)
                        | LymphNodeProduct::Pathogen(_)
                )
            )
            && !self.state.is_paused
            && !self.state.is_awaiting_resources
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeResource {
    Antigen(AntigenBinder),
    Body(Body),
    Protein(Protein),
}

impl LymphNodeResource {
    pub fn variants() -> impl Iterator<Item = Self> {
        let antigens = AntigenBinder::variants().map(Self::Antigen);
        let bodies = Body::variants().map(Self::Body);
        let proteins = Protein::variants().map(Self::Protein);

        antigens.chain(bodies).chain(proteins)
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Antigen(antigen) => antigen.asset_path(),
            Self::Body(body) => body.asset_path(),
            Self::Protein(protein) => protein.asset_path(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeTarget {
    Outside,
    LymphNode(Entity),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LymphNodeState {
    pub is_paused: bool,
    pub is_awaiting_resources: bool,
}

#[derive(Clone, Debug)]
pub enum LymphNodeProduct {
    Resource(LymphNodeResource),
    Leukocyte(Leukocyte),
    Pathogen(Pathogen),
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
                15.0,
                theme::z_index::LYMPH_NODE_COMPILATION_WARNING
                    - theme::z_index::LYMPH_NODE,
            ))
            .with_scale(vec3(LymphNode::SIZE, LymphNode::SIZE, 1.0));

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
                        size: vec2(LymphNode::SIZE * PHYSICS_SCALE * 1.5, 10.0),
                        ..Default::default()
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(vec3(0.0, 18.0, 0.1)),
                ..Default::default()
            })
            .insert(Self);
    }
}

#[derive(Component, Debug)]
pub struct LymphNodeConnection {
    pub source: Entity,
    pub source_pos: Vec2,
    pub target: Entity,
    pub target_pos: Vec2,
    pub wires: Vec<LymphNodeConnectionWire>,
    pub tt: f32,
}

impl LymphNodeConnection {
    pub fn new(
        source: Entity,
        source_pos: Vec2,
        target: Entity,
        target_pos: Vec2,
    ) -> Self {
        const WIRES: usize = 6;

        assert_ne!(source, target);
        assert_ne!(source_pos, target_pos);

        let wires = (0..WIRES).map(|idx| {
            let is_reverse = idx % 2 == 1;

            if is_reverse {
                LymphNodeConnectionWire::new(target_pos, source_pos, is_reverse)
            } else {
                LymphNodeConnectionWire::new(source_pos, target_pos, is_reverse)
            }
        });

        Self {
            source,
            source_pos,
            target,
            target_pos,
            wires: wires.collect(),
            tt: 0.0,
        }
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn().insert(self);
    }
}

#[derive(Component, Debug)]
pub struct LymphNodeConnectionWire {
    pub points: Vec<LymphNodeConnectionWirePoint>,
    pub tint_r: f32,
    pub tint_g: f32,
    pub tint_b: f32,
}

impl LymphNodeConnectionWire {
    pub fn new(source: Vec2, target: Vec2, is_reverse: bool) -> Self {
        const SEGMENT_LEN: f32 = 8.0;

        let mut rng = rand::thread_rng();
        let points_count = (source.distance(target) / SEGMENT_LEN) as i32;

        let dir = (source - target).normalize();
        let dirp = dir.perp();

        let points = (0..points_count).map(|idx| {
            let mut pos = source
                + (target - source) / (points_count as f32) * (idx as f32);

            pos += dirp * rng.gen_range(-5.0..5.0);

            LymphNodeConnectionWirePoint {
                pos,
                vel: Default::default(),
            }
        });

        let mut points = points.collect_vec();

        if !is_reverse {
            for width in [15.0, 13.0, 11.0, 9.0, 7.0, 5.0, 3.0, 1.0] {
                let at = target + dir * 70.0;

                let indicator_points = [
                    at,
                    at + dirp * width,
                    at - 20.0 * dir,
                    at - dirp * width,
                    at,
                ];

                points.extend(indicator_points.into_iter().map(|pos| {
                    LymphNodeConnectionWirePoint {
                        pos,
                        vel: Default::default(),
                    }
                }));
            }
        }

        Self {
            points,
            tint_r: rng.gen_range(-0.02..0.1),
            tint_g: rng.gen_range(-0.2..0.2),
            tint_b: rng.gen_range(-0.2..0.2),
        }
    }
}

#[derive(Debug)]
pub struct LymphNodeConnectionWirePoint {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(Component, Default)]
pub struct DeadLymphNodeConnection {
    pub tt: f32,
    pub started: bool,
}
