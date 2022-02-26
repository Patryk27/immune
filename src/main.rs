use bevy::prelude::*;

#[derive(Component, Debug)]
struct FactoryNode;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let sprite_handle = assets.load("placeholder_circle.png");

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0))
        .insert(FactoryNode);

    commands.spawn_bundle(SpriteBundle {
        texture: sprite_handle.clone(),
        transform: Transform::default(),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(factory_node_system)
        .run();
}

fn factory_node_system(mut query: Query<&mut FactoryNode>) {
    for _ in &mut query.iter_mut() {
        // Do something with factory nodes
    }
}
