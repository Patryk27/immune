mod discrete_map;
mod map;

use bevy::prelude::*;
use pathfinding::prelude::bfs;

pub use self::discrete_map::*;
pub use self::map::*;
use crate::pathfinding::discrete_map::FieldKinds;
use crate::systems::cell_node::LymphNode;
use crate::systems::units::Unit;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default()).add_system(refresh_map);
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

pub struct Pathfinder {
    map: DiscreteMap,
}

impl Pathfinder {
    pub fn new(map: &Map, pathseeker: Vec2, target: Vec2) -> Self {
        let map = DiscreteMap::new(map, pathseeker, target);

        Self { map }
    }

    pub fn path(&self) -> Vec<Vec2> {
        let path = bfs(&self.map, |map| map.successors(), |map| map.arrived());

        // self.debug_path(&path);

        path.into_iter()
            .flatten()
            .map(|map| map.pathseeker_pos())
            .collect()
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
