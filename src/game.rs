use bevy::prelude::*;

use crate::compiling::RecompileEvent;
use crate::level::{Level, LevelWave};
use crate::systems::bio::{
    LymphNode, LymphNodeState, LymphNodeTarget, Pathogen, PathogenKind,
};
use crate::systems::units::Alignment;
use crate::tutorial::TutorialState;

mod progress_bars;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::l1())
            .insert_resource(GameState::default())
            .insert_resource(TutorialState::default())
            .add_startup_system(setup)
            .add_system(progress)
            .add_system(game_over);

        progress_bars::initialize(app);
    }
}

pub struct GameState {
    pub tutorial: bool,
    pub seconds: f32,
    pub curr_wave_id: Option<usize>,
    pub next_wave_at: f32,
    pub game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tutorial: true,
            seconds: Default::default(),
            curr_wave_id: Default::default(),
            next_wave_at: Default::default(),
            game_over: false,
        }
    }
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
    if state.tutorial {
        return;
    }

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

fn game_over(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut state: ResMut<GameState>,
    lymph_nodes: Query<(&Transform, &Alignment, &LymphNode), With<LymphNode>>,
) {
    let player_owned_lymph_nodes = lymph_nodes
        .iter()
        .filter(|(_, alignment, _)| alignment.is_player())
        .count();

    if player_owned_lymph_nodes == 0 {
        state.game_over = true;
    }

    if !state.game_over {
        return;
    }

    let font = assets.load("fat-pixels.regular.ttf");

    let text_style = TextStyle {
        font,
        font_size: 45.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format!("Game Over"),
            text_style.clone(),
            text_alignment,
        ),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..Default::default()
    });
}
