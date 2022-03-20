use std::env;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

use super::physics::world_to_pixel;
use crate::pathfinding::{PathfindingPlugin, PathfindingState};
use crate::systems::enemy_ai::{self, EnemyAiEnabled};
use crate::systems::input::{SelectedUnits, SelectedUnitsChanged};
use crate::systems::units::{Alignment, Unit};
use crate::utils::DebugLinesExt;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if env::var("DEBUG").as_deref() == Ok("1") {
            app.insert_resource(DebugState::default())
                .add_system(draw_debug_window)
                .add_system(draw_ai_debug_window)
                .add_system(draw_vectors)
                .add_system(draw_pathfinder_paths)
                .add_system(draw_pathfinder_map);
        }
    }
}

#[derive(Default)]
pub struct DebugState {
    pub track_positions: bool,
    pub show_velocity_vectors: bool,
    pub show_force_vectors: bool,
    pub show_pathfinder_paths: bool,
    pub show_pathfinder_map: bool,
}

fn draw_debug_window(
    time: Res<Time>,
    mut state: ResMut<DebugState>,
    mut egui: ResMut<EguiContext>,
    mut units: Query<(&mut Unit, &mut Alignment)>,
    selected_units: Res<SelectedUnits>,
    mut commands: Commands,
) {
    egui::Window::new("Debug")
        .collapsible(true)
        .show(egui.ctx_mut(), |ctx| {
            let fps = 1.0 / time.delta_seconds();
            ctx.label(format!("FPS: ~{}", fps.round()));

            ctx.checkbox(&mut state.track_positions, "Track positions");
            ctx.checkbox(&mut state.show_velocity_vectors, "Show velocities");
            ctx.checkbox(&mut state.show_force_vectors, "Show forces");
            ctx.checkbox(
                &mut state.show_pathfinder_paths,
                "Show pathfinder's paths",
            );
            ctx.checkbox(
                &mut state.show_pathfinder_map,
                "Show pathfinder's map",
            );

            ctx.collapsing("Selected units", |ctx| {
                if ctx.button("Delete").clicked() {
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
        });
}

fn draw_ai_debug_window(
    ai_state: ResMut<enemy_ai::State>,
    mut ai_enabled: ResMut<EnemyAiEnabled>,
    mut egui: ResMut<EguiContext>,
    mut selected_units: ResMut<SelectedUnits>,
    mut selected_units_changed: EventWriter<SelectedUnitsChanged>,
) {
    egui::Window::new("Ai Debug").collapsible(true).show(
        egui.ctx_mut(),
        |ctx| {
            ctx.checkbox(&mut ai_enabled.0, "Ai Enabled");

            ctx.label("Combat groups:");
            egui::ScrollArea::vertical().show(ctx, |ctx| {
                for (idx, combat_group) in
                    ai_state.combat_groups.iter().enumerate()
                {
                    ctx.label(format!("Combat group #{idx}"));
                    ctx.label(format!("Units: {}", combat_group.units.len()));

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

fn draw_vectors(
    state: Res<DebugState>,
    query: Query<(
        &RigidBodyPositionComponent,
        &RigidBodyVelocityComponent,
        &RigidBodyForcesComponent,
    )>,
    mut lines: ResMut<DebugLines>,
) {
    if !state.show_velocity_vectors
        && !state.show_force_vectors
        && !state.track_positions
    {
        return;
    }

    for (position, velocity, force) in query.iter() {
        let position = position.position.translation.vector;
        let velocity = velocity.linvel;
        let force = force.force;

        if state.track_positions {
            let pos = world_to_pixel(position);

            lines.line(pos, pos + Vec3::X, 1.0);
        }

        if state.show_velocity_vectors {
            let start = world_to_pixel(position);
            let end = world_to_pixel(position + velocity);
            let color = Color::rgb(0.0, 1.0, 0.0);

            draw_arrow(&mut lines, start, end, color);
        }

        if state.show_force_vectors {
            let start = world_to_pixel(position);
            let end = world_to_pixel(position + force);
            let color = Color::rgb(1.0, 0.0, 0.0);

            draw_arrow(&mut lines, start, end, color);
        }
    }
}

fn draw_pathfinder_paths(
    state: Res<DebugState>,
    query: Query<(&Transform, &Unit)>,
    mut lines: ResMut<DebugLines>,
) {
    if !state.show_pathfinder_paths {
        return;
    }

    for (transform, unit) in query.iter() {
        let mut prev = transform.translation;

        if let Some(target) = unit.target {
            draw_arrow(&mut lines, prev, target.extend(0.0), Color::TOMATO);
        }

        for next in &unit.path {
            let next = next.extend(0.0);

            draw_arrow(&mut lines, prev, next, Color::FUCHSIA);

            prev = next;
        }
    }
}

fn draw_pathfinder_map(
    mut lines: ResMut<DebugLines>,
    state: Res<DebugState>,
    pathfinding: Res<PathfindingState>,
) {
    if !state.show_pathfinder_map {
        return;
    }

    for pos in pathfinding.obstacles() {
        let a = pos - PathfindingPlugin::FIELD_SIZE / 2.0;
        let b = pos + PathfindingPlugin::FIELD_SIZE / 2.0;

        lines.square(a, b).color(Color::RED).draw();
    }
}

fn draw_arrow(lines: &mut DebugLines, start: Vec3, end: Vec3, color: Color) {
    const WING_LENGTH: f32 = 10.0;

    let forward = (end - start).normalize() * WING_LENGTH;
    let angle = PI * 3.0 / 4.0;
    let left = Quat::from_axis_angle(Vec3::Z, angle);
    let right = Quat::from_axis_angle(Vec3::Z, -angle);

    let left = left * forward;
    let right = right * forward;

    lines.line_colored(start, end, 0.0, color);
    lines.line_colored(end, end + left, 0.0, color);
    lines.line_colored(end, end + right, 0.0, color);
}
