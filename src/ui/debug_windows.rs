use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::systems::debug::DebugState;
use crate::systems::enemy_ai::{self, EnemyAiEnabled};
use crate::systems::input::{SelectedUnits, SelectedUnitsChanged};
use crate::systems::units::{Alignment, Unit};

pub fn initialize(app: &mut App) {
    app.insert_resource(crate::systems::debug::DebugState::default())
        .add_system(ai_debug_window)
        .add_system(debug_options_window);
}

fn debug_options_window(
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

fn ai_debug_window(
    ai_state: ResMut<enemy_ai::State>,
    mut ai_enabled: ResMut<EnemyAiEnabled>,
    mut egui: ResMut<EguiContext>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
) {
    egui::Window::new("Ai Debug Window").collapsible(true).show(
        egui.ctx_mut(),
        |ctx| {
            ctx.checkbox(&mut ai_enabled.0, "Ai Enabled");

            ctx.label("Combat groups:");
            egui::ScrollArea::vertical().show(ctx, |ctx| {
                for (idx, combat_group) in
                    ai_state.combat_groups.iter().enumerate()
                {
                    ctx.label(format!("Combat group #{idx}"));
                    ctx.label(format!(
                        "Units in group: {}",
                        combat_group.units.len()
                    ));

                    if ctx.button("Select units").clicked() {
                        selected_units.selected_units =
                            combat_group.units.clone();
                        selected_units_changed.send(SelectedUnitsChanged);
                    }

                    ctx.separator();
                }
            });
        },
    );
}
