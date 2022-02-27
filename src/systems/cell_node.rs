use std::str::FromStr;

use bevy::math::vec2;
use bevy::prelude::*;

use super::units::Unit;
use crate::compiler::Compiler;
use crate::map::Map;

const MAP: &str = include_str!("./map.toml");

mod antigen;
mod antigen_binder;
mod body;
mod cell;
mod leukocyte;
mod lymph_node;
mod pathogen;
mod protein;

pub use self::antigen::*;
pub use self::antigen_binder::*;
pub use self::body::*;
pub use self::cell::*;
pub use self::leukocyte::*;
pub use self::lymph_node::*;
pub use self::pathogen::*;
pub use self::protein::*;

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(process)
        .add_system(track_cells_position);
}

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let map = Map::from_str(MAP).unwrap();

    for (idx, lymph_node_item) in map.lymph_nodes.iter().enumerate() {
        let lymph_node = {
            let body = if idx % 2 == 0 {
                Body::Hexagon
            } else {
                Body::Circle
            };

            LymphNode {
                time_to_spawn: 1.0,
                timer: 1.0,
                lhs: Some(LymphNodeInput::Body(body)),
                rhs: Some(LymphNodeInput::Binder(AntigenBinder::new(
                    Antigen::Triangle,
                ))),
            }
        };

        lymph_node.spawn(&mut commands, &assets, lymph_node_item.pos);
    }

    commands.insert_resource(map);
    // ---

    let mut x = -300.0;
    let y = 300.0;

    for antigen in [Antigen::Rectangle, Antigen::Semicircle, Antigen::Triangle]
    {
        for body in [Body::Circle, Body::Hexagon] {
            Pathogen {
                antigen,
                body,
                kind: PathogenKind::Virus,
            }
            .spawn(&mut commands, &assets, vec2(x, y));

            x += 125.0;
        }
    }
}

pub fn track_cells_position(
    mut map: ResMut<Map>,
    query: Query<(&Unit, &Transform)>,
) {
    map.cell_nodes = query
        .iter()
        .map(|(_, transform)| transform.translation.into())
        .collect()
}

pub fn process(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<(&mut LymphNode, &Transform)>,
) {
    let compiler = Compiler::new();

    for (mut lymph_node, transform) in &mut query.iter_mut() {
        let lymph_node = &mut *lymph_node;

        let product = if let Some(product) = lymph_node.output(&compiler) {
            product
        } else {
            continue;
        };

        lymph_node.timer -= time.delta_seconds();

        if lymph_node.timer <= 0.0 {
            lymph_node.timer = lymph_node.time_to_spawn;

            match product {
                LymphNodeOutput::Leukocyte(leukocyte) => {
                    leukocyte.spawn(
                        &mut commands,
                        &assets,
                        transform.translation.truncate(),
                    );
                }
            }
        }
    }
}
