use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

use super::input::MousePos;

const SPEED: f32 = 500.0;

// On Windows the wheel events come in with values between -4 and 4
// in the browser the values are -400 to 400
#[cfg(target_arch = "wasm32")]
const ZOOM_SPEED: f32 = 0.0032;
#[cfg(not(target_arch = "wasm32"))]
const ZOOM_SPEED: f32 = 0.32;

const MAX_ZOOM: f32 = 10.0;

#[derive(Default)]
pub struct State {
    is_moving_up: bool,
    is_moving_down: bool,
    is_moving_left: bool,
    is_moving_right: bool,
}

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(movement)
        .add_system(mouse);
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(State::default());

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));
}

pub fn mouse(
    mut query: Query<(&mut Transform, &Camera, &mut OrthographicProjection)>,
    mouse: Res<Input<MouseButton>>,
    mut cursor: EventReader<MouseMotion>,
    mut wheel: EventReader<MouseWheel>,
    mouse_pos: Res<MousePos>,
) {
    let (mut transform, _, mut ortho) = query.single_mut();

    if mouse.pressed(MouseButton::Middle) {
        for event in cursor.iter() {
            let mut delta = event.delta.extend(0.0);
            delta.x *= -1.0;

            transform.translation += delta * ortho.scale;
        }
    }

    for event in wheel.iter() {
        let a = ortho.scale;

        ortho.scale =
            f32::clamp(ortho.scale - event.y * ZOOM_SPEED, 1.0, MAX_ZOOM);

        let b = ortho.scale;
        let change = b - a;

        transform.translation -= (mouse_pos.1 * change).extend(0.0);
    }
}

pub fn movement(
    state: Res<State>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Camera, &OrthographicProjection)>,
) {
    let (mut transform, _, ortho) = query.single_mut();
    let speed = SPEED * time.delta_seconds() * ortho.scale;

    if state.is_moving_up {
        transform.translation.y += speed;
    }

    if state.is_moving_down {
        transform.translation.y -= speed;
    }

    if state.is_moving_left {
        transform.translation.x -= speed;
    }

    if state.is_moving_right {
        transform.translation.x += speed;
    }
}

pub fn keyboard_input(
    mut state: ResMut<State>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            match key_code {
                KeyCode::Up | KeyCode::W => {
                    state.is_moving_up = event.state.is_pressed()
                }
                KeyCode::Down | KeyCode::S => {
                    state.is_moving_down = event.state.is_pressed()
                }
                KeyCode::Left | KeyCode::A => {
                    state.is_moving_left = event.state.is_pressed()
                }
                KeyCode::Right | KeyCode::D => {
                    state.is_moving_right = event.state.is_pressed()
                }
                _ => (),
            };
        }
    }
}
