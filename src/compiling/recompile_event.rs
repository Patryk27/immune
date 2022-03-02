use std::collections::HashSet;

use bevy::prelude::*;

use super::{CompilationWarning, Compiler};
use crate::systems::cell_node::{
    DeadLymphNodeConnection, LymphNode, LymphNodeConnection, LymphNodeInput,
};

#[derive(Clone, Debug)]
pub struct RecompileEvent;

pub(super) fn compile(
    mut events: EventReader<RecompileEvent>,
    mut nodes: Query<(Entity, &mut LymphNode, &Children)>,
    mut warnings: Query<(&mut CompilationWarning, &mut Visibility)>,
) {
    if events.iter().next().is_none() {
        return;
    }

    let mut compiler = Compiler::default();

    for (entity, node, _) in nodes.iter() {
        compiler.add(entity, node);
    }

    for (entity, output) in compiler.compile() {
        let (_, mut node, children) = nodes.get_mut(entity).unwrap();

        node.output = output;

        for child in children.iter() {
            if let Ok((mut warn, mut warn_vis)) = warnings.get_mut(*child) {
                if !warn_vis.is_visible && output.is_none() {
                    warn.tt = 0.0;
                }

                warn_vis.is_visible = output.is_none();
            }
        }
    }
}

pub(super) fn link(
    mut commands: Commands,
    mut events: EventReader<RecompileEvent>,
    nodes: Query<(Entity, &LymphNode, &Transform)>,
    connections: Query<
        (Entity, &LymphNodeConnection),
        Without<DeadLymphNodeConnection>,
    >,
) {
    if events.iter().next().is_none() {
        return;
    }

    let mut existing_connections = HashSet::new();
    let mut required_connections = HashSet::new();

    for (_, connection) in connections.iter() {
        existing_connections
            .insert((connection.source_entity, connection.target_entity));
    }

    for (target_entity, target_node, &target_transform) in nodes.iter() {
        for source in [target_node.lhs, target_node.rhs] {
            let source_entity =
                if let Some(LymphNodeInput::External(Some(source))) = source {
                    source
                } else {
                    continue;
                };

            required_connections.insert((source_entity, target_entity));
            required_connections.insert((target_entity, source_entity));

            let (_, _, &source_transform) = nodes.get(source_entity).unwrap();

            let source = source_transform.translation.truncate();
            let target = target_transform.translation.truncate();

            for _ in 0..3 {
                commands.spawn().insert(LymphNodeConnection::new(
                    source,
                    source_entity,
                    target,
                    target_entity,
                ));
            }

            for _ in 0..3 {
                commands.spawn().insert(LymphNodeConnection::new(
                    target,
                    target_entity,
                    source,
                    source_entity,
                ));
            }
        }
    }

    let unnecessary_connections =
        existing_connections.difference(&required_connections);

    for &(source_entity, target_entity) in unnecessary_connections {
        let connection_entities =
            connections.iter().filter_map(|(entity, connection)| {
                if connection.source_entity == source_entity
                    && connection.target_entity == target_entity
                {
                    Some(entity)
                } else {
                    None
                }
            });

        for entity in connection_entities {
            commands
                .entity(entity)
                .insert(DeadLymphNodeConnection::default());
        }
    }
}
