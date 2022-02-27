use std::path::Path;
use std::str::FromStr;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub lymph_nodes: Vec<LymphNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LymphNode {
    pub pos: Vec2,
}

impl Map {
    pub fn load(p: impl AsRef<Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(p)?;

        Self::from_str(&content)
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Map = toml::from_str(s)?;

        Ok(map)
    }
}
