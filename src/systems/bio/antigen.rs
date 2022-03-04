use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use super::{Body, CellFadeIn};
use crate::systems::bio::Cell;
use crate::systems::physics::PHYSICS_SCALE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Antigen {
    Rectangle,
    Semicircle,
    Triangle,
}

impl Antigen {
    pub const SIZE: f32 = 0.04;

    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Rectangle, Self::Semicircle, Self::Triangle].into_iter()
    }

    pub fn color(parent: Color, a: u8) -> Color {
        let [h, s, mut l, _] = parent.as_hlsa_f32();

        l /= 2.0;

        Color::hsla(h, s, l, (a as f32) / 255.0)
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
        parent_body: Body,
        parent_color: Color,
    ) {
        self.spawn_ex(
            assets,
            entity,
            parent_body,
            parent_color,
            self.asset_path(),
        );
    }

    pub fn spawn_ex(
        self,
        assets: &AssetServer,
        entity: &mut ChildBuilder,
        parent_body: Body,
        parent_color: Color,
        asset_path: &str,
    ) {
        let texture = assets.load(asset_path);

        for transform in Self::transforms(parent_body) {
            let sprite = Sprite {
                color: Self::color(parent_color, 0),
                ..Default::default()
            };

            let sprite = SpriteBundle {
                sprite,
                transform,
                texture: texture.clone(),
                ..Default::default()
            };

            entity.spawn_bundle(sprite).insert(CellFadeIn::default());
        }
    }

    fn transforms(body: Body) -> impl Iterator<Item = Transform> {
        const DISTANCE: f32 = Cell::SIZE * PHYSICS_SCALE;

        let sides = match body {
            Body::Circle => 4,
            Body::Hexagon => 3,
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

    pub fn random(rng: &mut impl Rng) -> Self {
        let idx = rng.gen::<usize>() % 3;

        match idx {
            0 => Self::Rectangle,
            1 => Self::Semicircle,
            2 => Self::Triangle,
            _ => unreachable!(),
        }
    }
}
