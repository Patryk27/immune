use std::path::Path;
use std::str::FromStr;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub factory_nodes: Vec<FactoryNode>,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FactoryNode {
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
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
