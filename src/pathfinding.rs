mod discrete_map;
mod map;

use std::collections::VecDeque;

use bevy::prelude::*;
use instant::Instant;
use pathfinding::prelude::astar;

pub use self::discrete_map::*;
pub use self::map::*;
use crate::level::{Level, LevelPoint};
use crate::systems::bio::{LymphNode, Wall, WallFadeOut};
use crate::systems::units::Unit;

pub struct PathfindingPlugin;

impl PathfindingPlugin {
    pub const FIELD_SIZE: f32 = Level::FIELD_SIZE;

    // TODO(pwy) to avoid confusion, we should return e.g. PathfindingPoint
    pub fn world_to_local(pos: Vec2) -> LevelPoint {
        Level::world_to_local(pos)
    }

    pub fn local_to_world(pos: LevelPoint) -> Vec2 {
        Level::local_to_world(pos)
    }
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingState::default())
            .add_event::<LevelLayoutChanged>()
            .add_event::<NavigateUnit>()
            .add_system(process_level_changed_event)
            .add_system(process_navigate_unit_event)
            .add_system(process_queue);
    }
}

#[derive(Default)]
pub struct PathfindingState {
    map: Map,
    queue: VecDeque<(Entity, Vec2, Vec2)>,
    budget_ms: i32,
}

impl PathfindingState {
    pub fn obstacles(&self) -> impl Iterator<Item = Vec2> + '_ {
        let lymph_nodes = self.map.lymph_nodes.iter().map(|node| node.pos);
        let walls = self.map.walls.iter().map(|wall| wall.pos);

        lymph_nodes
            .chain(walls)
            .map(PathfindingPlugin::local_to_world)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LevelLayoutChanged;

#[derive(Clone, Debug)]
pub struct NavigateUnit {
    pub entity: Entity,
    pub target: Vec2,
}

fn process_level_changed_event(
    mut events: EventReader<LevelLayoutChanged>,
    mut state: ResMut<PathfindingState>,
    level: Res<Level>,
    lymph_nodes: Query<&LymphNode>,
    walls: Query<&Wall, Without<WallFadeOut>>,
) {
    if events.iter().next().is_none() {
        return;
    }

    state.map.lymph_nodes = lymph_nodes
        .iter()
        .map(|node| MapLymphNode { pos: node.pos })
        .collect();

    state.map.walls =
        walls.iter().map(|wall| MapWall { pos: wall.pos }).collect();

    let (min_x, min_y, max_x, max_y) = level.bounds();

    state.map.bounds = MapBounds {
        min_x,
        min_y,
        max_x,
        max_y,
    };
}

fn process_navigate_unit_event(
    mut events: EventReader<NavigateUnit>,
    mut state: ResMut<PathfindingState>,
    mut units: Query<(&Transform, &mut Unit)>,
) {
    for NavigateUnit { entity, target } in events.iter() {
        state.queue.retain(|(entity2, _, _)| entity2 != entity);

        if let Ok((transform, mut unit)) = units.get_mut(*entity) {
            state.queue.push_back((
                *entity,
                transform.translation.truncate(),
                *target,
            ));

            unit.target = Some(*target);
        } else {
            // Unit must've been destroyed before we were able to process the
            // event (or someone requested navigation for a non-Unit)
        }
    }
}

fn process_queue(
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
    pub fn new(map: &Map, pathseeker: Vec2, target: Vec2) -> Option<Self> {
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
            .map(PathfindingPlugin::local_to_world)
            .collect();

        Some(Self { path })
    }

    pub fn into_path(self) -> Vec<Vec2> {
        self.path
    }
}
