use bevy::prelude::*;

use super::{AntigenBinder, Body, Cell};

#[derive(Component, Clone, Copy, Debug)]
pub struct Leukocyte {
    pub body: Body,
    pub binder: AntigenBinder,
    pub kind: LeukocyteKind,
    pub props: LeukocyteProps,
}

impl Leukocyte {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        pos: Vec2,
        vel: Vec2,
    ) {
        Cell::Leukocyte(self).spawn(commands, assets, pos, vel);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeukocyteKind {
    // Cager, TODO(pwy) post-MVP
    Killer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeukocyteProps {
    pub hp: u32,
}
