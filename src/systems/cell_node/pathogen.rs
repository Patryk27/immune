use bevy::prelude::*;

use super::{Antigen, Body, Cell};

#[derive(Component, Clone, Copy, Debug)]
pub struct Pathogen {
    pub body: Body,
    pub antigen: Antigen,
    pub kind: PathogenKind,
}

impl Pathogen {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        pos: Vec2,
        vel: Vec2,
    ) {
        Cell::Pathogen(self).spawn(commands, assets, pos, vel);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathogenKind {
    Virus,
}
