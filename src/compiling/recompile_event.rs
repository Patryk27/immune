use std::collections::HashSet;

use bevy::prelude::*;

use super::Compiler;
use crate::systems::bio::{
    DeadLymphNodeConnection, LymphNode, LymphNodeConnection, LymphNodeInput,
    LymphNodeWarning,
};

#[derive(Clone, Debug)]
pub struct RecompileEvent;

pub(super) fn compile(
    mut events: EventReader<RecompileEvent>,
    mut nodes: Query<(Entity, &mut LymphNode, &Children)>,
    mut warnings: Query<&mut LymphNodeWarning>,
) {
    if events.iter().next().is_none() {
        return;
    }

    let mut compiler = Compiler::default();

    for (entity, node, _) in nodes.iter() {
        compiler.add(entity, node);
    }

    for (entity, output, state, function) in compiler.compile() {
        let (_, mut node, children) = nodes.get_mut(entity).unwrap();

        node.output = output;
        node.function = function;
        node.state = state;

        for child in children.iter() {
            if let Ok(mut warn) = warnings.get_mut(*child) {
                warn.set(if node.state.paused {
                    Some("lymph-node.state.paused.png")
                } else if node.state.awaiting_resources {
                    Some("lymph-node.state.awaiting-resources.png")
                } else if node.output.is_none() {
                    Some("lymph-node.state.error.png")
                } else {
                    None
                });
            }
        }
    }
}

pub(super) fn link(
    mut commands: Commands,
    mut events: EventReader<RecompileEvent>,
    nodes: Query<(Entity, &LymphNode, &Transform)>,
    connnections: Query<
        (Entity, &LymphNodeConnection),
        Without<DeadLymphNodeConnection>,
    >,
) {
    if events.iter().next().is_none() {
        return;
    }

    let mut existing_connections = HashSet::new();
    let mut required_connections = HashSet::new();

    for (_, connection) in connnections.iter() {
        existing_connections.insert((connection.source, connection.target));
    }

    for (target, target_node, &target_transform) in nodes.iter() {
        for source in [target_node.lhs, target_node.rhs] {
            let source =
                if let Some(LymphNodeInput::External(Some(source))) = source {
                    source
                } else {
                    continue;
                };

            required_connections.insert((source, target));

            let (_, _, &source_transform) = nodes.get(source).unwrap();

            LymphNodeConnection::new(
                source,
                source_transform.translation.truncate(),
                target,
                target_transform.translation.truncate(),
            )
            .spawn(&mut commands);
        }
    }

    let unnecessary_connections =
        existing_connections.difference(&required_connections);

    for &(source, target) in unnecessary_connections {
        let connections =
            connnections.iter().filter_map(|(entity, connection)| {
                if connection.source == source && connection.target == target {
                    Some(entity)
                } else {
                    None
                }
            });

        for connection in connections {
            commands
                .entity(connection)
                .insert(DeadLymphNodeConnection::default());
        }
    }
}
