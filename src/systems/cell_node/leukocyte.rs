use bevy::prelude::*;

use super::{AntigenBinder, Body, Cell};

#[derive(Component, Clone, Debug)]
pub struct Leukocyte {
    pub body: Body,
    pub binder: AntigenBinder,
    pub kind: LeukocyteKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeukocyteKind {
    // Cager, TODO(pwy) post-MVP
    Killer,
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
