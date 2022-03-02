mod compilation_warning;
mod compiler;
mod recompile_event;

use bevy::prelude::*;

pub use self::compilation_warning::CompilationWarning;
pub(self) use self::compiler::*;
pub use self::recompile_event::RecompileEvent;

pub struct CompilingPlugin;

impl Plugin for CompilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecompileEvent>()
            .add_system(recompile_event::compile)
            .add_system(recompile_event::link)
            .add_system(compilation_warning::blink);
    }
}
