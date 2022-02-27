use std::f32::consts::TAU;

use bevy::prelude::*;

use super::Body;

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

    pub fn spawn_ex(
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
