mod compiler;

use bevy::prelude::*;

use self::compiler::*;
use crate::systems::cell_node::LymphNode;

pub struct CompilingPlugin;

impl Plugin for CompilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(recompile_lymph_nodes);
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeedsRecompiling;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompilationWarning;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
struct HasCompilationWarning;

fn recompile_lymph_nodes(
    mut commands: Commands,
    mut dirty_lymph_nodes: Query<
        (Entity, &mut LymphNode, &Children),
        With<NeedsRecompiling>,
    >,
    mut clean_lymph_nodes: Query<
        (Entity, &mut LymphNode, &Children),
        Without<NeedsRecompiling>,
    >,
    mut warnings: Query<&mut Visibility, With<CompilationWarning>>,
) {
    if dirty_lymph_nodes.is_empty() {
        return;
    }

    let mut compiler = Compiler::default();

    let all_nodes = dirty_lymph_nodes.iter().chain(clean_lymph_nodes.iter());

    for (entity, node, _) in all_nodes {
        compiler.add(entity, node);
    }

    for (entity, output) in compiler.compile() {
        let (_, mut node, children) = dirty_lymph_nodes
            .get_mut(entity)
            .unwrap_or_else(|_| clean_lymph_nodes.get_mut(entity).unwrap());

        node.output = output;

        // TODO(pwy) feels hacky
        for child in children.iter() {
            if let Ok(mut warning) = warnings.get_mut(*child) {
                warning.is_visible = output.is_none();
            }
        }
    }

    for (entity, _, _) in dirty_lymph_nodes.iter_mut() {
        commands.entity(entity).remove::<NeedsRecompiling>();
    }
}
