use bevy::prelude::*;

use super::{Antigen, Body};

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
