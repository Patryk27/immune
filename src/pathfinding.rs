mod discrete_map;
mod map;

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use pathfinding::prelude::bfs;

pub use self::discrete_map::*;
pub use self::map::*;
use crate::pathfinding::discrete_map::FieldKinds;
use crate::systems::cell_node::LymphNode;
use crate::systems::units::Unit;

type Pathseeker = Vec2;
type Target = Vec2;

pub struct PathfindingPlugin;

#[derive(Default)]
pub struct PathseekersQueue {
    queued: HashMap<Entity, (Pathseeker, Target)>,
}

impl PathseekersQueue {
    pub fn insert(
        &mut self,
        entity: Entity,
        pathseeker: Pathseeker,
        target: Target,
    ) {
        self.queued.insert(entity, (pathseeker, target));
    }
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default())
            .insert_resource(PathseekersQueue::default())
            .add_system(refresh_map)
            .add_system(spawn_pathfinder_task)
            .add_system(handle_pathfinder_task);
    }
}

fn refresh_map(
    mut map: ResMut<Map>,
    lymph_nodes: Query<&Transform, With<LymphNode>>,
    units: Query<&Transform, With<Unit>>,
) {
    map.lymph_nodes = lymph_nodes
        .iter()
        .map(|transform| MapLymphNode {
            pos: transform.translation.truncate(),
            size: 80.0, // TODO shouldn't be hard-coded
        })
        .collect();

    map.units = units
        .iter()
        .map(|transform| MapUnit {
            pos: transform.translation.truncate(),
            size: 50.0, // TODO shouldn't be hard-coded
        })
        .collect();
}

fn spawn_pathfinder_task(
    mut commands: Commands,
    mut state: ResMut<PathseekersQueue>,
    map: Res<Map>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    let queue = state.queued.clone();

    for (entity, (pathfinder, target)) in queue {
        let map = map.clone();
        state.queued.remove(&entity);

        let task = thread_pool
            .spawn(async move { Pathfinder::new(map, pathfinder, target) });

        commands.entity(entity).insert(task);
    }
}

fn handle_pathfinder_task(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut Unit, &mut Task<Pathfinder>)>,
) {
    for (entity, mut unit, mut task) in transform_tasks.iter_mut() {
        if let Some(pathfinder) =
            future::block_on(future::poll_once(&mut *task))
        {
            let path = pathfinder.path();
            unit.set_path(path);

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<Task<Pathfinder>>();
        }
    }
}

#[derive(Component)]
pub struct Pathfinder {
    map: DiscreteMap,
    path: Vec<Vec2>,
}

impl Pathfinder {
    pub fn new(map: Map, pathseeker: Pathseeker, target: Target) -> Self {
        let map = DiscreteMap::new(&map, pathseeker, target);
        let path = bfs(&map, |map| map.successors(), |map| map.arrived());

        let path = path
            .into_iter()
            .flatten()
            .map(|map| map.pathseeker_pos())
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
