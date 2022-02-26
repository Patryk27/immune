use bevy::prelude::*;
use bevy_smud::SmudPlugin;
use unfair_advantage::systems::cell_node;

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(1, 8, 90)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SmudPlugin)
        .add_startup_system(setup)
        .add_startup_system(cell_node::setup)
        .add_system(cell_node::factory_node_system)
        .add_system(cell_node::brownian_motion_system)
        .run();
}
