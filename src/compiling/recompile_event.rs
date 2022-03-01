use bevy::prelude::*;

use super::{CompilationWarning, Compiler};
use crate::systems::cell_node::LymphNode;

#[derive(Clone, Debug)]
pub struct RecompileEvent;

pub(super) fn handle(
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
