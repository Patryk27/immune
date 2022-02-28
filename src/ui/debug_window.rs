use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::systems::debug::DebugState;

pub fn system(
    time: Res<Time>,
    mut state: ResMut<DebugState>,
    mut egui: ResMut<EguiContext>,
) {
    egui::Window::new("Debug Window").collapsible(true).show(
        egui.ctx_mut(),
        |ctx| {
            ctx.label(format!(
                "Time since startup {:?}",
                time.time_since_startup()
            ));
            let fps = 1.0 / time.delta_seconds();
            ctx.label(format!("est. FPS: {}", fps.round()));

            ctx.checkbox(&mut state.show_motion_vectors, "Show motion vectors");
            ctx.checkbox(&mut state.show_force_vectors, "Show force vectors");
            ctx.checkbox(&mut state.track_position, "Track position");
        },
    );
}
