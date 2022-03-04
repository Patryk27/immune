use bevy::prelude::*;

use super::{Antigen, Body};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AntigenBinder(pub Antigen);

impl AntigenBinder {
    pub fn new(antigen: Antigen) -> Self {
        Self(antigen)
    }

    pub fn variants() -> impl Iterator<Item = Self> {
        Antigen::variants().map(Self)
    }

    pub fn asset_path(&self) -> &'static str {
        match self.0 {
            Antigen::Rectangle => "antigen-binder.rectangle.png",
            Antigen::Semicircle => "antigen-binder.semicircle.png",
            Antigen::Triangle => "antigen-binder.triangle.png",
        }
    }

    pub fn spawn(
        self,
        assets: &AssetServer,
        entity: &mut ChildBuilder,
        parent_body: Body,
        parent_color: Color,
    ) {
        self.0.spawn_ex(
            assets,
            entity,
            parent_body,
            parent_color,
            self.asset_path(),
        );
    }
}
