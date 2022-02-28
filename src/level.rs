use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Level {
    #[serde(rename = "Setup")]
    pub setup: LevelSetup,
    #[serde(rename = "Wave")]
    pub waves: Vec<LevelWave>,
}

impl Level {
    pub fn l1() -> Self {
        Self::new(include_str!("../levels/1.toml"))
    }

    fn new(str: &str) -> Self {
        toml::from_str(str).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelSetup {
    #[serde(rename = "LymphNode")]
    pub lymph_nodes: Vec<LevelLymphNode>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelWave {
    #[serde(rename = "StartsAt")]
    pub starts_at: u64,
    #[serde(rename = "Virus")]
    pub viruses: Vec<LevelVirus>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelVirus {
    #[serde(rename = "Pos")]
    pub pos: Vec2,
    #[serde(rename = "Vel")]
    pub vel: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LevelLymphNode {
    #[serde(rename = "Pos")]
    pub pos: Vec2,
}
