use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

const SPEED: f32 = 500.0;

pub struct State {
    is_moving_up: bool,
    is_moving_down: bool,
    is_moving_left: bool,
    is_moving_right: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_moving_up: false,
            is_moving_down: false,
            is_moving_left: false,
            is_moving_right: false,
        }
    }
}

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(movement);
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(State::default());

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));
}

pub fn movement(
    state: Res<State>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    let (mut transform, _) = query.single_mut();

    let speed = SPEED * time.delta_seconds();

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
