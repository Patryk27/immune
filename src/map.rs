use std::path::Path;
use std::str::FromStr;

use bevy::math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub lymph_nodes: Vec<LymphNode>,
    pub cell_nodes: Vec<CellNode>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LymphNode {
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellNode {
    pub pos: Vec2,
}

impl From<Vec3> for CellNode {
    fn from(v: Vec3) -> Self {
        Self {
            pos: v.truncate()
        }
    }
}