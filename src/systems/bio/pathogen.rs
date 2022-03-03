use bevy::prelude::*;
use rand::prelude::SliceRandom;

use super::{Antigen, Body, Cell};

#[derive(Component, Clone, Copy, Debug)]
pub struct Pathogen {
    pub body: Body,
    pub antigen: Antigen,
    pub kind: PathogenKind,
}

impl Pathogen {
    pub fn color(a: u8) -> Color {
        let colors = vec![
            (52, 153, 255),  // light-blue~ish
            (27, 3, 179),    // dark-blue~ish
            (255, 100, 192), // pink~ish
            (88, 0, 208),    // purple~ish
            (244, 80, 52),   // orange~ish
            (255, 192, 19),  // yellow~ish
            (64, 253, 213),  // green~ish
        ];

        let &(r, g, b) = colors.choose(&mut rand::thread_rng()).unwrap();

        Color::rgba_u8(r, g, b, a)
    }

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
