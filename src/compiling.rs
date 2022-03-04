mod compiler;
mod recompile_event;

use bevy::prelude::*;

use self::compiler::*;
pub use self::recompile_event::RecompileEvent;

pub struct CompilingPlugin;

impl Plugin for CompilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecompileEvent>()
            .add_system(recompile_event::compile)
            .add_system(recompile_event::link);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompilationWarning {
    NodeIsPaused,
    NodeIsAwaitingResources,
    NodeHasNoProduct,
    NodeHasNoChild,
    Infected,
}

impl CompilationWarning {
    pub fn description(self) -> &'static str {
        match self {
            Self::NodeIsPaused => {
                "[!] Node does not produce anything because it is paused."
            }
            Self::NodeIsAwaitingResources => {
                "[!] Node does not produce anything because its parent is paused."
            }
            Self::NodeHasNoProduct => "[!] Node does not produce anything because it is misconfigured (i.e. it uses an illegal combination of resources).",
            Self::NodeHasNoChild => "[!] Node does not produce anything because it must be linked with another node first.",
            Self::Infected => "[!] Node is infected and cannot be controlled.",
        }
    }

    pub fn asset_path(self) -> &'static str {
        match self {
            Self::NodeIsPaused => "lymph-node.state.paused.png",
            Self::NodeIsAwaitingResources => {
                "lymph-node.state.awaiting-resources.png"
            }
            Self::NodeHasNoProduct | Self::NodeHasNoChild => {
                "lymph-node.state.error.png"
            }
            Self::Infected => "lymph-node.state.infected.png",
        }
    }
}
