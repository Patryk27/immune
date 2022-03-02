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
