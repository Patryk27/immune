use bevy::prelude::*;

use super::physics::PHYSICS_SCALE;
use crate::level::Level;

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
        .add_system(progress_lymph_nodes);
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let level = Level::l1();

    for lymph_node in &level.setup.lymph_nodes {
        LymphNode {
            time_to_spawn: 3.0,
            timer: 1.0,
            lhs: None,
            rhs: None,
            output: None,
        }
        .spawn(&mut commands, &assets, lymph_node.pos);
    }

    commands.insert_resource(level);
}

fn progress_lymph_nodes(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<(Entity, &mut LymphNode, &Transform)>,
) {
    for (_, mut lymph_node, transform) in &mut query.iter_mut() {
        let lymph_node = &mut *lymph_node;

        let output = if let Some(output) = &lymph_node.output {
            output
        } else {
            continue;
        };

        lymph_node.timer -= time.delta_seconds();

        if lymph_node.timer <= 0.0 {
            lymph_node.timer = lymph_node.time_to_spawn;

            match output {
                LymphNodeOutput::Leukocyte(leukocyte) => {
                    leukocyte.spawn(
                        &mut commands,
                        &assets,
                        transform.translation.truncate() / PHYSICS_SCALE,
                    );
                }
            }
        }
    }
}
