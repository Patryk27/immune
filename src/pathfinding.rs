mod discrete_map;
mod map;

use std::collections::VecDeque;

use bevy::prelude::*;
use instant::Instant;
use pathfinding::prelude::astar;

pub use self::discrete_map::*;
pub use self::map::*;
use crate::pathfinding::discrete_map::FieldKinds;
use crate::systems::bio::{LymphNode, Wall};
use crate::systems::units::Unit;

type Pathseeker = Vec2;
type Target = Vec2;

pub struct PathfindingPlugin;

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
    lymph_nodes: Query<&Transform, With<LymphNode>>,
    walls: Query<&Transform, With<Wall>>,
) {
    state.map.lymph_nodes = lymph_nodes
        .iter()
        .map(|transform| MapLymphNode {
            pos: transform.translation.truncate(),
        })
        .collect();

    state.map.walls = walls
        .iter()
        .map(|transform| MapWall {
            pos: transform.translation.truncate(),
        })
        .collect();
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

        let pathfinder = Pathfinder::new(&state.map, pathseeker, target);
        let path = pathfinder.path();
        unit.set_path(path);
    }

    state.budget_ms -= tt.elapsed().as_millis() as i32;
}

#[derive(Component)]
pub struct Pathfinder {
    map: DiscreteMap,
    path: Vec<Vec2>,
}

impl Pathfinder {
    pub fn new(map: &Map, pathseeker: Pathseeker, target: Target) -> Self {
        let map = DiscreteMap::new(map, pathseeker, target, None);
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
            .collect();

        Self { map, path }
    }

    pub fn path(self) -> Vec<Vec2> {
        self.path
    }

    #[allow(dead_code)]
    fn debug_path(&self, path: &Option<Vec<DiscreteMap>>) {
        let mut path_map = self.map.clone();
        for map in path.iter().flatten() {
            path_map.mark(map.pathseeker(), FieldKinds::Path)
        }

        path_map.mark(self.map.pathseeker(), FieldKinds::Pathseeker);
        path_map.mark(self.map.target(), FieldKinds::Target);

        println!("{}", path_map);
    }
}
