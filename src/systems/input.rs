use bevy::prelude::*;
use bevy_egui::EguiContext;

pub use self::collider::*;
pub use self::selector::*;
use super::draw_square;
use super::units::Unit;
use crate::pathfinding::PathseekersQueue;

mod collider;
mod selector;
mod selectors;
mod unit_selection;

pub use self::unit_selection::{SelectedUnits, SelectedUnitsChanged};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(unit_selection::UnitSelectionPlugin)
            .insert_resource(InputState::default())
            .insert_resource(MousePos::default())
            .add_system(track_mouse_position)
            .add_system(movement_command)
            .add_system(selectors::track_selector_hovers)
            .add_system(selectors::update_selector_highlights)
            .add_system(selectors::animate_selectors);
    }
}

#[derive(Default)]
pub struct MousePos(pub Vec2);

pub struct InputState {
    pub is_dragging: bool,
    pub drag_start_pos: Vec2,
    pub hovered_entity: Option<Entity>,
    pub highlight_zoom: f32,
    pub highlight_zoom_dir: f32,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_start_pos: Vec2::ZERO,
            hovered_entity: Default::default(),
            highlight_zoom: 1.0,
            highlight_zoom_dir: 1.0,
        }
    }
}

fn track_mouse_position(
    mut egui: ResMut<EguiContext>,
    mut mouse_pos: ResMut<MousePos>,
    wnds: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        return;
    }

    let (camera, camera_transform) = camera.single();
    let wnd = wnds.get(camera.window).unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix()
            * camera.projection_matrix.inverse();

        mouse_pos.0 = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
    }
}

fn movement_command(
    mut egui: ResMut<EguiContext>,
    mouse_pos: Res<MousePos>,
    selected_units: Res<SelectedUnits>,
    mut pathfinding_queue: ResMut<PathseekersQueue>,
    mouse: Res<Input<MouseButton>>,
    mut units: Query<(Entity, &mut Unit, &Transform)>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        return;
    }

    if mouse.just_pressed(MouseButton::Right) {
        for unit in selected_units.selected_units.iter() {
            if let Ok((entity, mut unit, transform)) = units.get_mut(*unit) {
                let target = mouse_pos.0;

                pathfinding_queue.insert(
                    entity,
                    transform.translation.truncate(),
                    target,
                );

                unit.target = Some(target);
            }
        }
    }
}
