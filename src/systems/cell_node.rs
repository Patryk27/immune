use std::f32::consts::TAU;
use std::str::FromStr;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::highlight::Highlight;
use super::units::Unit;
use crate::compiler::Compiler;
use crate::map::Map;

const MAP: &str = include_str!("./map.toml");

#[derive(Component, Clone, Debug)]
pub struct LymphNode {
    pub time_to_spawn: f32,
    pub timer: f32,
    pub lhs: Option<LymphNodeInput>,
    pub rhs: Option<LymphNodeInput>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphNodeInput {
    Binder(AntigenBinder),
    Body(Body),
    Protein(Protein),
    External(Entity),
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

#[derive(Clone, Debug)]
pub enum LymphNodeOutput {
    Leukocyte(Leukocyte),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protein {
    Dumbbell,
    Star,
}

impl Protein {
    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Dumbbell, Self::Star].into_iter()
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Dumbbell => "protein.dumbbell.png",
            Self::Star => "protein.star.png",
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Leukocyte {
    pub body: Body,
    pub binder: AntigenBinder,
    pub kind: LeukocyteKind,
}

impl Leukocyte {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        at: Vec2,
    ) {
        Cell::Leukocyte(self).spawn(commands, assets, at);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeukocyteKind {
    // Cager, TODO(pwy) post-MVP
    Killer,
}

#[derive(Component, Clone, Debug)]
pub struct Pathogen {
    pub antigen: Antigen,
    pub body: Body,
    pub kind: PathogenKind,
}

impl Pathogen {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        at: Vec2,
    ) {
        Cell::Pathogen(self).spawn(commands, assets, at);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathogenKind {
    Virus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Antigen {
    Rectangle,
    Semicircle,
    Triangle,
}

impl Antigen {
    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Rectangle, Self::Semicircle, Self::Triangle].into_iter()
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Rectangle => "antigen.rectangle.png",
            Self::Semicircle => "antigen.semicircle.png",
            Self::Triangle => "antigen.triangle.png",
        }
    }

    pub fn spawn(
        self,
        assets: &AssetServer,
        entity: &mut ChildBuilder,
        body: Body,
    ) {
        self.spawn_ex(
            assets,
            entity,
            body,
            self.asset_path(),
            Color::rgb_u8(128, 0, 0),
        );
    }

    fn spawn_ex(
        self,
        assets: &AssetServer,
        entity: &mut ChildBuilder,
        body: Body,
        asset_path: &str,
        color: Color,
    ) {
        let texture = assets.load(asset_path);

        for transform in Self::transforms(body) {
            let sprite = Sprite {
                color,
                ..Default::default()
            };

            let sprite = SpriteBundle {
                sprite,
                transform,
                texture: texture.clone(),
                ..Default::default()
            };

            entity.spawn_bundle(sprite);
        }
    }

    fn transforms(body: Body) -> impl Iterator<Item = Transform> {
        const DISTANCE: f32 = 40.0;

        let sides = match body {
            Body::Circle => 3,
            Body::Hexagon => 2,
        };

        (0..sides).map(move |side| {
            let angle = (side as f32) * TAU / (sides as f32);

            let transform_rot =
                Transform::from_rotation(Quat::from_rotation_z(angle));

            let transform_pos =
                Transform::from_translation(Vec3::new(0.0, DISTANCE, -0.01));

            (transform_rot * transform_pos).with_scale(Vec3::splat(0.1))
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AntigenBinder(Antigen);

impl AntigenBinder {
    pub fn new(antigen: Antigen) -> Self {
        Self(antigen)
    }

    pub fn variants() -> impl Iterator<Item = Self> {
        Antigen::variants().map(Self)
    }

    pub fn spawn(
        self,
        assets: &AssetServer,
        entity: &mut ChildBuilder,
        body: Body,
    ) {
        self.0.spawn_ex(
            assets,
            entity,
            body,
            self.asset_path(),
            Color::rgb_u8(128, 128, 128),
        );
    }

    pub fn asset_path(self) -> &'static str {
        match self.0 {
            Antigen::Rectangle => "antigen-binder.rectangle.png",
            Antigen::Semicircle => "antigen-binder.semicircle.png",
            Antigen::Triangle => "antigen-binder.triangle.png",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Body {
    Circle,
    Hexagon,
}

impl Body {
    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Circle, Self::Hexagon].into_iter()
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Circle => "body.circle.png",
            Self::Hexagon => "body.hexagon.png",
        }
    }
}

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
                    angular_damping: 0.0,
                    linear_damping: 0.99999,
                }),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::ball(25.0)),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .insert(Unit::default());

        match self {
            Cell::Leukocyte(cell) => {
                entity.insert((*cell).clone());
            }
            Cell::Pathogen(cell) => {
                entity.insert((*cell).clone());
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

// ---

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let map = Map::from_str(MAP).unwrap();

    for (idx, lymph_node_item) in map.lymph_nodes.iter().enumerate() {
        let lymph_node = {
            let body = if idx % 2 == 0 {
                Body::Hexagon
            } else {
                Body::Circle
            };

            LymphNode {
                time_to_spawn: 1.0,
                timer: 1.0,
                lhs: Some(LymphNodeInput::Body(body)),
                rhs: Some(LymphNodeInput::Binder(AntigenBinder::new(
                    Antigen::Triangle,
                ))),
            }
        };

        lymph_node.spawn(&mut commands, &assets, lymph_node_item.pos);
    }

    commands.insert_resource(map);
    // ---

    let mut x = -300.0;
    let y = 300.0;

    for antigen in [Antigen::Rectangle, Antigen::Semicircle, Antigen::Triangle]
    {
        for body in [Body::Circle, Body::Hexagon] {
            Pathogen {
                antigen,
                body,
                kind: PathogenKind::Virus,
            }
            .spawn(&mut commands, &assets, vec2(x, y));

            x += 125.0;
        }
    }
}

pub fn track_cells_position(
    mut map: ResMut<Map>,
    query: Query<(&Unit, &Transform)>,
) {
    map.cell_nodes = query.iter().map(|(_, transform)| transform.translation.into()).collect()
}

pub fn process(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<(&mut LymphNode, &Transform)>,
) {
    let compiler = Compiler::new();

    for (mut lymph_node, transform) in &mut query.iter_mut() {
        let lymph_node = &mut *lymph_node;

        let product = if let Some(product) = lymph_node.output(&compiler) {
            product
        } else {
            continue;
        };

        lymph_node.timer -= time.delta_seconds();

        if lymph_node.timer <= 0.0 {
            lymph_node.timer = lymph_node.time_to_spawn;

            match product {
                LymphNodeOutput::Leukocyte(leukocyte) => {
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
