use bevy::prelude::*;
use unfair_advantage::systems::cell_node;

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(cell_node::setup)
        .add_system(cell_node::factory_node_system)
        .add_system(cell_node::brownian_motion_system)
        .run();
}
