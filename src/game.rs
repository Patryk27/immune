use bevy::prelude::*;

use crate::level::{Level, LevelWave};
use crate::systems::cell_node::{
    Antigen, Body, LymphNode, Pathogen, PathogenKind,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::l1())
            .insert_resource(GameState::default())
            .add_startup_system(setup)
            .add_system(progress);
    }
}

#[derive(Default)]
struct GameState {
    seconds: f32,
    curr_wave_id: Option<usize>,
    next_wave_at: f32,
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut state: ResMut<GameState>,
    level: Res<Level>,
) {
    for lymph_node in &level.setup.lymph_nodes {
        LymphNode {
            time_to_spawn: 3.0,
            timer: 1.0,
            lhs: None,
            rhs: None,
            output: None,
        }
        .spawn(&mut commands, &assets, lymph_node.pos);
    }

    state.next_wave_at = level.waves[0].starts_at as _;
}

fn progress(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut state: ResMut<GameState>,
    level: Res<Level>,
) {
    state.seconds += time.delta_seconds();

    if state.seconds >= state.next_wave_at {
        let curr_wave_id = state.curr_wave_id.map_or(0, |wave_id| wave_id + 1);

        if let Some(wave) = level.waves.get(curr_wave_id) {
            spawn_wave(&mut commands, &assets, wave);
        }

        if let Some(wave) = level.waves.get(curr_wave_id + 1) {
            state.next_wave_at = wave.starts_at as _;
        } else {
            // TODO proceed to the next level?
            state.next_wave_at = f32::MAX;
        }

        state.curr_wave_id = Some(curr_wave_id);
    }
}

fn spawn_wave(commands: &mut Commands, assets: &AssetServer, wave: &LevelWave) {
    for virus in &wave.viruses {
        // TODO(pwy) apply velocity

        Pathogen {
            body: Body::Hexagon,
            antigen: Antigen::Semicircle,
            kind: PathogenKind::Virus,
        }
        .spawn(commands, assets, virus.pos);
    }
}
