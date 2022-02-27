use bevy::prelude::*;

use super::{Antigen, Body, Cell};

#[derive(Component, Clone, Debug)]
pub struct Pathogen {
    pub antigen: Antigen,
    pub body: Body,
    pub kind: PathogenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathogenKind {
    Virus,
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
