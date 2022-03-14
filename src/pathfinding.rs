mod discrete_map;
mod map;

use std::collections::VecDeque;

use bevy::math::vec2;
use bevy::prelude::*;
use instant::Instant;
use pathfinding::prelude::astar;

pub use self::discrete_map::*;
pub use self::map::*;
use crate::level::{Level, LevelPoint};
use crate::systems::bio::{LymphNode, Wall};
use crate::systems::physics::PHYSICS_SCALE;
use crate::systems::units::Unit;

type Pathseeker = Vec2;
type Target = Vec2;

pub struct PathfindingPlugin;

impl PathfindingPlugin {
    // TOOD(pwy) perhaps `Level` would be a better place for it?
    fn world_to_level(pos: Vec2) -> LevelPoint {
        let pos = pos / Wall::SIZE / PHYSICS_SCALE;

        LevelPoint::new(pos.x as i32, pos.y as i32)
    }

    fn level_to_world(pos: LevelPoint) -> Vec2 {
        vec2(
            (pos.x as f32) * Wall::SIZE * PHYSICS_SCALE,
            (pos.y as f32) * Wall::SIZE * PHYSICS_SCALE,
        )
    }
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingState::default())
            .add_system(refresh_map)
            .add_system(hande_queue);
    }
}

#[derive(Default)]
pub struct PathfindingState {
    pub map: Map,
    pub queue: VecDeque<(Entity, Pathseeker, Target)>,
    pub budget_ms: i32,
}

impl PathfindingState {
    pub fn add(
        &mut self,
        entity: Entity,
        pathseeker: Pathseeker,
        target: Target,
    ) {
        self.queue.retain(|(entity2, _, _)| *entity2 != entity);
        self.queue.push_back((entity, pathseeker, target));
    }
}

fn refresh_map(
    mut state: ResMut<PathfindingState>,
    level: Res<Level>,
    lymph_nodes: Query<&Transform, With<LymphNode>>,
    walls: Query<&Transform, With<Wall>>,
) {
    state.map.lymph_nodes = lymph_nodes
        .iter()
        .map(|transform| MapLymphNode {
            pos: PathfindingPlugin::world_to_level(
                transform.translation.truncate(),
            ),
        })
        .collect();

    // ---

    state.map.walls = walls
        .iter()
        .map(|transform| MapWall {
            pos: PathfindingPlugin::world_to_level(
                transform.translation.truncate(),
            ),
        })
        .collect();

    // ---

    let (min_x, min_y, max_x, max_y) = level.bounds();

    state.map.bounds = MapBounds {
        min_x,
        min_y,
        max_x,
        max_y,
    };
}

fn hande_queue(
    mut state: ResMut<PathfindingState>,
    mut units: Query<&mut Unit>,
) {
    state.budget_ms += 12;

    if state.budget_ms < 0 {
        // Previous frame's path-finding took more than the allowed budget,
        // recovering
        return;
    }

    if state.budget_ms >= 12 {
        // Accumulating budget would be pointless, since we want to keep the
        // algorithm working in real-time
        state.budget_ms = 12;
    }

    if state.queue.is_empty() {
        return;
    }

    let tt = Instant::now();

    while (tt.elapsed().as_millis() as i32) < state.budget_ms {
        let (entity, pathseeker, target) =
            if let Some(item) = state.queue.pop_front() {
                item
            } else {
                break;
            };

        let mut unit = if let Ok(unit) = units.get_mut(entity) {
            unit
        } else {
            continue;
        };

        let path = Pathfinder::new(&state.map, pathseeker, target)
            .map(|pathfinder| pathfinder.into_path())
            .unwrap_or_default();

        unit.set_path(path);
    }

    state.budget_ms -= tt.elapsed().as_millis() as i32;
}

#[derive(Component)]
pub struct Pathfinder {
    path: Vec<Vec2>,
}

impl Pathfinder {
    pub fn new(
        map: &Map,
        pathseeker: Pathseeker,
        target: Target,
    ) -> Option<Self> {
        let map = DiscreteMap::new(map, pathseeker, target)?;
        let start = map.start();

        let path = astar(
            &start,
            |pos| map.successors(**pos),
            |pos| map.heuristic(**pos),
            |pos| map.success(**pos),
        );

        let path = path
            .map(|(path, _)| path)
            .into_iter()
            .flatten()
            .map(|idx| map.idx_to_pos(*idx))
            .map(PathfindingPlugin::level_to_world)
            .collect();

        Some(Self { path })
    }

    pub fn into_path(self) -> Vec<Vec2> {
        self.path
    }
}
