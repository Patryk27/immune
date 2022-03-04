use std::collections::HashSet;

use bevy::prelude::*;

use super::{CompilationWarning, Compiler};
use crate::systems::bio::{
    DeadLymphNodeConnection, LymphNode, LymphNodeConnection, LymphNodeProduct,
    LymphNodeTarget, LymphNodeWarning,
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

    for (entity, (product, parent, state)) in compiler.compile() {
        let (_, mut node, children) = nodes.get_mut(entity).unwrap();

        node.product = product;
        node.parent = parent;
        node.state = state;

        node.warning =
            if matches!(node.product, Some(LymphNodeProduct::Pathogen(_))) {
                Some(CompilationWarning::Infected)
            } else if node.state.is_paused {
                Some(CompilationWarning::NodeIsPaused)
            } else if node.state.is_awaiting_resources {
                Some(CompilationWarning::NodeIsAwaitingResources)
            } else if node.product.is_none() {
                Some(CompilationWarning::NodeHasNoProduct)
            } else if matches!(
                node.product,
                Some(LymphNodeProduct::Resource(_))
            ) && !matches!(node.target, LymphNodeTarget::LymphNode(_))
            {
                Some(CompilationWarning::NodeHasNoChild)
            } else {
                None
            };

        for child in children.iter() {
            if let Ok(mut warn) = warnings.get_mut(*child) {
                warn.set(node.warning.map(|warn| warn.asset_path()));
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

    for (source, source_node, &source_transform) in nodes.iter() {
        let target =
            if let LymphNodeTarget::LymphNode(target) = source_node.target {
                target
            } else {
                continue;
            };

        required_connections.insert((source, target));

        let (_, _, &target_transform) = nodes.get(target).unwrap();

        LymphNodeConnection::new(
            source,
            source_transform.translation.truncate(),
            target,
            target_transform.translation.truncate(),
        )
        .spawn(&mut commands);
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
