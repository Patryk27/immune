pub mod compiling;
pub mod game;
pub mod level;
pub mod pathfinding;
pub mod systems;
pub mod ui;

pub(crate) mod z_index {
    pub const CELL: f32 = 1.0;
    pub const LYMPH_NODE: f32 = 0.9;
    pub const LYMPH_NODE_COMPILATION_WARNING: f32 = 1.1;
}
