use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use super::units::Unit;

#[derive(Component)]
struct SelectionRect;

pub struct State {
    _selected_units: Vec<Entity>,

    drag_start_pos: Vec3,
    drag_end_pos: Vec3,
    is_dragging: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            _selected_units: vec![],
            is_dragging: false,
            drag_end_pos: Vec3::ZERO,
            drag_start_pos: Vec3::ZERO,
        }
    }
}

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup).add_system(system);
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(State::default());
}

pub fn system(
    mut state: ResMut<State>,
    mouse_button_input_events: EventReader<MouseButtonInput>,
    cursor_moved_events: EventReader<CursorMoved>,
    mut _units: Query<(Entity, &Transform, &Unit)>,
    mut lines: ResMut<DebugLines>,
    camera: Query<(&OrthographicProjection, &Camera, &Transform)>,
) {
    update_state(&mut state, cursor_moved_events, mouse_button_input_events);

    if !state.is_dragging {
        return;
    }

    let (ortho, _, camera) = camera.single();

    let start_pos = screen_to_world_point(camera, ortho, state.drag_start_pos);
    let end_pos = screen_to_world_point(camera, ortho, state.drag_end_pos);

    draw_square(&mut lines, start_pos, end_pos);
}

fn update_state(
    state: &mut State,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    for event in cursor_moved_events.iter() {
        state.drag_end_pos = Vec3::new(event.position.x, event.position.y, 0.0);
    }

    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            match (state.is_dragging, event.state.is_pressed()) {
                // Drag end
                (true, false) => {
                    state.is_dragging = false;
                }
                // Drag start
                (false, true) => {
                    state.is_dragging = true;
                    state.drag_start_pos = state.drag_end_pos;
                }
                _ => (),
            }
        }
    }
}

fn draw_square(lines: &mut DebugLines, start_point: Vec3, end_point: Vec3) {
    let right = Vec3::new(end_point.x - start_point.x, 0.0, 0.0);
    let up = Vec3::new(0.0, end_point.y - start_point.y, 0.0);

    lines.line(start_point, start_point + right, 0.0);
    lines.line(start_point, start_point + up, 0.0);
    lines.line(end_point, end_point - right, 0.0);
    lines.line(end_point, end_point - up, 0.0);
}

// Probably not compatible with zooming or other projections
fn screen_to_world_point(
    camera: &Transform,
    ortho: &OrthographicProjection,
    point: Vec3,
) -> Vec3 {
    let ortho_offset = Vec3::new(ortho.left, ortho.bottom, 0.0);

    point + camera.translation + ortho_offset
}
