use bevy::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub lymph_nodes: Vec<MapLymphNode>,
    pub units: Vec<MapUnit>,
}

#[derive(Clone, Debug)]
pub struct MapLymphNode {
    pub pos: Vec2,
    pub size: f32,
}

#[derive(Clone, Debug)]
pub struct MapUnit {
    pub pos: Vec2,
    pub size: f32,
}
