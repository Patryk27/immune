use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use super::{Antigen, Body, Cell};

#[derive(Component, Clone, Copy, Debug)]
pub struct Pathogen {
    pub body: Body,
    pub antigen: Antigen,
    pub kind: PathogenKind,
}

impl Pathogen {
    pub fn random() -> Self {
        let mut rng = thread_rng();

        Self {
            body: Body::random(&mut rng),
            antigen: Antigen::random(&mut rng),
            kind: PathogenKind::Virus,
        }
    }

    pub fn color(a: u8) -> Color {
        let mut rng = rand::thread_rng();

        let colors = vec![
            (52, 153, 255),  // light-blue~ish
            (27, 3, 179),    // dark-blue~ish
            (255, 100, 192), // pink~ish
            (88, 0, 208),    // purple~ish
            (244, 80, 52),   // orange~ish
            (255, 192, 19),  // yellow~ish
            (64, 253, 213),  // green~ish
        ];

        let &(r, g, b) = colors.choose(&mut rng).unwrap();
        let [mut h, s, l, a] = Color::rgba_u8(r, g, b, a).as_hlsa_f32();

        h += rng.gen_range(-40.0..40.0);

        Color::hsla(h, s, l, a)
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
