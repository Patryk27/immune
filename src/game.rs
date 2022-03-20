mod progress_bars;

use bevy::prelude::*;
use instant::{Duration, Instant};

use crate::compiling::RecompileEvent;
use crate::level::{Level, LevelPoint, LevelWaveOp};
use crate::pathfinding::LevelLayoutChanged;
use crate::systems::bio::{
    LymphNode, LymphNodeState, LymphNodeTarget, Wall, WallFadeIn, WallFadeOut,
};
use crate::systems::units::Alignment;
use crate::tutorial::TutorialState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::start())
            .insert_resource(GameState::default())
            .insert_resource(TutorialState::default())
            .add_system(progress)
            .add_system(game_over);

        progress_bars::initialize(app);
    }
}

pub struct GameState {
    pub tutorial: bool,
    pub seconds: f32,
    pub vm: LevelVm,
    pub game_over: bool,
}

pub enum LevelVm {
    Idle,
    AwaitingStart {
        at: Instant,
    },
    Asleep {
        until: Option<Instant>,
        op_idx: usize,
    },
    AwaitingWaveEnd,
    AwaitingWaveStart {
        at: Instant,
    },
}

impl Default for LevelVm {
    fn default() -> Self {
        Self::Idle
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tutorial: true,
            seconds: Default::default(),
            vm: LevelVm::default(),
            game_over: false,
        }
    }
}

fn progress(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
    mut state: ResMut<GameState>,
    walls: Query<(Entity, &Wall)>,
    mut level: ResMut<Level>,
    mut level_changed_tx: EventWriter<LevelLayoutChanged>,
    mut recompile_event_tx: EventWriter<RecompileEvent>,
    lymph_nodes: Query<&Alignment, With<LymphNode>>,
) {
    if state.game_over {
        return;
    }

    loop {
        match state.vm {
            LevelVm::Idle => {
                return;
            }

            LevelVm::AwaitingStart { at } => {
                if Instant::now() < at {
                    return;
                }

                state.vm = LevelVm::Asleep {
                    until: None,
                    op_idx: 0,
                };
            }

            LevelVm::Asleep { until, op_idx } => {
                if let Some(until) = until {
                    if Instant::now() < until {
                        return;
                    }
                }

                let op = &level.wave.ops[op_idx];
                let mut sleep = true;

                match op {
                    LevelWaveOp::AddWall { x, y } => {
                        Wall {
                            pos: LevelPoint::new(*x, *y),
                        }
                        .spawn(&mut commands, &assets);

                        level_changed_tx.send(LevelLayoutChanged);
                    }

                    LevelWaveOp::RemoveWall { x, y } => {
                        sleep = false;

                        for (entity, wall) in walls.iter() {
                            if wall.pos.x == *x && wall.pos.y == *y {
                                sleep = true;

                                commands.entity(entity).remove::<WallFadeIn>();

                                commands
                                    .entity(entity)
                                    .insert(WallFadeOut::default());

                                level_changed_tx.send(LevelLayoutChanged);
                            }
                        }
                    }

                    LevelWaveOp::AddLymphNode { x, y, alignment } => {
                        LymphNode {
                            pos: LevelPoint::new(*x, *y),
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
                            *alignment,
                        );

                        level_changed_tx.send(LevelLayoutChanged);
                        recompile_event_tx.send(RecompileEvent);
                    }
                }

                state.vm = if level.wave.ops.get(op_idx + 1).is_some() {
                    let until = if sleep {
                        Some(Instant::now() + Duration::from_millis(2))
                    } else {
                        None
                    };

                    LevelVm::Asleep {
                        until,
                        op_idx: op_idx + 1,
                    }
                } else {
                    LevelVm::AwaitingWaveEnd
                };
            }

            LevelVm::AwaitingWaveEnd => {
                if lymph_nodes.iter().all(|a| a.is_player()) {
                    state.vm = LevelVm::AwaitingWaveStart {
                        at: Instant::now() + Duration::from_secs(25),
                    };
                } else {
                    return;
                }
            }

            LevelVm::AwaitingWaveStart { at } => {
                if Instant::now() <= at {
                    return;
                }

                level.progress();

                state.vm = LevelVm::Asleep {
                    until: Some(Instant::now()),
                    op_idx: 0,
                };
            }
        }
    }
}

fn game_over(
    mut state: ResMut<GameState>,
    lymph_nodes: Query<(&Transform, &Alignment, &LymphNode), With<LymphNode>>,
) {
    let enemy_owned_lymph_nodes = lymph_nodes
        .iter()
        .filter(|(_, alignment, _)| alignment.is_enemy())
        .count();

    let player_owned_lymph_nodes = lymph_nodes
        .iter()
        .filter(|(_, alignment, _)| alignment.is_player())
        .count();

    if enemy_owned_lymph_nodes > 0 && player_owned_lymph_nodes == 0 {
        state.game_over = true;
    }
}
