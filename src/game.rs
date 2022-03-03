use bevy::prelude::*;

use crate::compiling::RecompileEvent;
use crate::level::{Level, LevelWave};
use crate::systems::bio::{
    LymphNode, LymphNodeState, LymphNodeTarget, Pathogen, PathogenKind,
};

mod progress_bars;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::l1())
            .insert_resource(GameState::default())
            .add_startup_system(setup)
            .add_system(progress);

        progress_bars::initialize(app);
    }
}

#[derive(Default)]
struct GameState {
    pub seconds: f32,
    pub curr_wave_id: Option<usize>,
    pub next_wave_at: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
    mut state: ResMut<GameState>,
    level: Res<Level>,
    mut recompile_event_tx: EventWriter<RecompileEvent>,
) {
    for lymph_node in &level.setup.lymph_nodes {
        LymphNode {
            resource: None,
            target: LymphNodeTarget::Outside,
            product: None,
            parent: None,
            warning: None,
            state: LymphNodeState {
                is_paused: false,
                is_awaiting_resources: false,
            },
            production_tt: 0.0,
        }
        .spawn(
            &mut commands,
            &mut meshes,
            &mut materials,
            &assets,
            lymph_node.pos,
        );
    }

    state.next_wave_at = level.waves[0].starts_at as _;

    recompile_event_tx.send(RecompileEvent);
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
        for _ in 0..virus.count {
            Pathogen {
                body: virus.body,
                antigen: virus.antigen,
                kind: PathogenKind::Virus,
            }
            .spawn(commands, assets, virus.pos, virus.vel);
        }
    }
}
