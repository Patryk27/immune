use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protein {
    Dumbbell,
    Star,
}

impl Protein {
    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Dumbbell, Self::Star].into_iter()
    }

    pub fn color() -> Color {
        Color::GOLD
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Dumbbell => "protein.dumbbell.png",
            Self::Star => "protein.star.png",
        }
    }
}
