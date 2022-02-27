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
}
