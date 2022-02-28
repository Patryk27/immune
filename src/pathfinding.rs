mod discrete_map;
mod map;

use bevy::prelude::*;

pub use self::discrete_map::DiscreteMap;
pub use self::map::*;
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
            size: 100.0, // TODO shouldn't be hard-coded
        })
        .collect();

    map.units = units
        .iter()
        .map(|transform| MapUnit {
            pos: transform.translation.truncate(),
            size: 60.0, // TODO shouldn't be hard-coded
        })
        .collect();
}
