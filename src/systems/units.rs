use bevy::prelude::*;

const MAX_SPEED: f32 = 5.0;
const FORCE_FACTOR: f32 = 1.0;
const STOPPING_FORCE_FACTOR: f32 = 2.0;

const MOVEMENT_STRETCH_FACTOR: f32 = 1.4;
const MOVEMENT_SQUEEZE_FACTOR: f32 = 0.6;

const MAX_HEALTH: f32 = 1.0;
const BASE_DAMAGE: f32 = 0.25; // By default a cell can take 4 hits

mod animate;
mod combat;
mod movement;

#[derive(Component)]
pub struct Unit {
    // TODO(dzejkop): Should be enum, target can be unit, etc.
    pub target: Option<Vec2>,
    pub path: Vec<Vec2>,
    pub step: usize,
    pub health: f32,
    pub alignment: Alignment,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alignment {
    Player,
    Enemy,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            target: Default::default(),
            path: Default::default(),
            step: Default::default(),
            health: MAX_HEALTH,
            alignment: Alignment::Player,
        }
    }
}

impl Unit {
    pub fn set_path(&mut self, path: Vec<Vec2>) {
        self.step = 0;
        self.path = path;
    }
}

pub fn initialize(app: &mut App) {
    app.add_system(movement::system)
        .add_system(animate::system)
        .add_system(combat::system);
}
