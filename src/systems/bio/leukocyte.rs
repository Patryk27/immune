use bevy::prelude::*;

use super::{AntigenBinder, Body, Cell, Protein};

#[derive(Component, Clone, Debug)]
pub struct Leukocyte {
    pub body: Body,
    pub binder: AntigenBinder,
    pub kind: LeukocyteKind,
    pub props: LeukocyteProps,
    pub proteins: Vec<Protein>,
}

impl Leukocyte {
    pub fn color(a: u8) -> Color {
        Color::rgba_u8(255, 255, 255, a)
    }

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeukocyteProps {
    pub hp: f32,
}
