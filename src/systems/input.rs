use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use super::units::Unit;
use super::highlight::Highlight;

pub struct State {
    selected_units: Vec<Entity>,

    is_dragging: bool,
    drag_start_pos: Vec3,

    current_mouse_pos: Vec3,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected_units: vec![],
            is_dragging: false,
            current_mouse_pos: Vec3::ZERO,
            drag_start_pos: Vec3::ZERO,
        }
    }
}

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(track_mouse_position)
        .add_system(selection)
        .add_system(highlight_selection)
        .add_system(command);
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(State::default());
}

pub fn track_mouse_position(
    mut state: ResMut<State>,
    camera: Query<(&OrthographicProjection, &Camera, &Transform)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let (ortho, _, camera) = camera.single();

    for event in cursor_moved_events.iter() {
        let pos = Vec3::new(event.position.x, event.position.y, 0.0);
        state.current_mouse_pos = screen_to_world_point(camera, ortho, pos);
    }
}

pub fn command(
    state: Res<State>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut units: Query<&mut Unit>,
) {
    for event in mouse_button_input_events.iter() {
        if event.state.is_pressed() && event.button == MouseButton::Right {
            for unit in state.selected_units.iter() {
                units.get_mut(*unit).unwrap().target =
                    Some(state.current_mouse_pos);
            }
        }
    }
}

pub fn selection(
    mut state: ResMut<State>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    units: Query<(Entity, &Transform, &Unit)>,
    mut lines: ResMut<DebugLines>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            match (state.is_dragging, event.state.is_pressed()) {
                // Drag end
                (true, false) => {
                    state.is_dragging = false;

                    state.selected_units = units
                        .iter()
                        .filter(|(_, transform, _)| {
                            is_unit_within_selection(
                                transform,
                                state.drag_start_pos,
                                state.current_mouse_pos,
                            )
                        })
                        .map(|(entity, _, _)| entity)
                        .collect();
                }
                // Drag start
                (false, true) => {
                    state.is_dragging = true;
                    state.drag_start_pos = state.current_mouse_pos;
                }
                _ => (),
            }
        }
    }

    if !state.is_dragging {
        return;
    }

    draw_square(&mut lines, state.drag_start_pos, state.current_mouse_pos);
}


pub fn highlight_selection(
    state: ResMut<State>,
    units: Query<(Entity, &Unit, &Children)>,
    mut highlights: Query<(Entity, &Highlight, &mut Visibility)>,
) {
    let mut selected_children: Vec<Entity> = Vec::new();

    for selected_entity in state.selected_units.iter() {
        let (_, _, children) = units.get(*selected_entity).unwrap();

        selected_children.extend(children.iter());
    }

    for (entity, _, mut visibility) in highlights.iter_mut() {
        if selected_children.contains(&entity) {
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}



fn is_unit_within_selection(
    unit: &Transform,
    start_pos: Vec3,
    end_pos: Vec3,
) -> bool {
    let min_x = start_pos.x.min(end_pos.x);
    let max_x = start_pos.x.max(end_pos.x);
    let min_y = start_pos.y.min(end_pos.y);
    let max_y = start_pos.y.max(end_pos.y);

    unit.translation.x > min_x
        && unit.translation.x < max_x
        && unit.translation.y > min_y
        && unit.translation.y < max_y
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
