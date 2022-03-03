use std::f32::consts::TAU;

use bevy::prelude::*;
use serde::Deserialize;

use super::{Body, CellFadeIn};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Antigen {
    Rectangle,
    Semicircle,
    Triangle,
}

impl Antigen {
    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Rectangle, Self::Semicircle, Self::Triangle].into_iter()
    }

    pub fn color(parent: Color, a: u8) -> Color {
        let [r, g, b, _] = parent.as_rgba_f32();

        Color::rgba_u8(
            (255.0 * r / 2.0) as _,
            (255.0 * g / 2.0) as _,
            (255.0 * b / 2.0) as _,
            a,
        )
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
        const DISTANCE: f32 = 40.0;

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
}
