use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Body {
    Circle,
    Hexagon,
}

impl Body {
    pub fn variants() -> impl Iterator<Item = Self> {
        [Self::Circle, Self::Hexagon].into_iter()
    }

    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Circle => "body.circle.png",
            Self::Hexagon => "body.hexagon.png",
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let idx = rng.gen::<usize>() % 2;

        match idx {
            0 => Self::Circle,
            1 => Self::Hexagon,
            _ => unreachable!(),
        }
    }
}
