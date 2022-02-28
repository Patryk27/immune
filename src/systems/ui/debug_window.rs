use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::systems::debug::DebugState;

pub fn system(mut state: ResMut<DebugState>, mut egui: ResMut<EguiContext>) {
    egui::Window::new("Debug Window").collapsible(true).show(
        egui.ctx_mut(),
        |ctx| {
            ctx.checkbox(&mut state.show_motion_vectors, "Show motion vectors");
            ctx.checkbox(&mut state.show_force_vectors, "Show force vectors");
            ctx.checkbox(&mut state.track_position, "Track position");
        },
    );
}
