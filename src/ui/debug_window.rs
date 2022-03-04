use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::systems::debug::DebugState;
use crate::systems::input::SelectedUnits;
use crate::systems::units::{Alignment, Unit};

pub fn system(
    time: Res<Time>,
    mut state: ResMut<DebugState>,
    mut egui: ResMut<EguiContext>,
    mut units: Query<(&mut Unit, &mut Alignment)>,
    selected_units: Res<SelectedUnits>,
    mut commands: Commands,
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
            ctx.checkbox(&mut state.show_pathfinding, "Show pathfinding");
            ctx.checkbox(&mut state.track_position, "Track position");
            ctx.checkbox(
                &mut state.draw_obstacles_from_map,
                "Draw obstacles from map",
            );

            ctx.collapsing("Unit ops", |ctx| {
                if ctx.button("Delete selected units").clicked() {
                    for unit in selected_units.selected_units.iter() {
                        commands.entity(*unit).despawn_recursive();
                    }
                }

                if ctx.button("Flip alignment").clicked() {
                    for unit in selected_units.selected_units.iter() {
                        if let Ok((_, mut alignment)) = units.get_mut(*unit) {
                            alignment.flip();
                        }
                    }
                }
            });
        },
    );
}
