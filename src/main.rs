use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Component, Debug)]
struct FactoryNode {
    time_to_spawn: f32,
    timer: f32,
}

#[derive(Component, Debug)]
struct Node;

#[derive(Component, Debug)]
struct Cell;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let sprite_handle = assets.load("placeholder_circle.png");

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));

    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_handle.clone(),
            transform: Transform::default(),
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FactoryNode {
            time_to_spawn: 1.0,
            timer: 1.0,
        });

    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_handle.clone(),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Node);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(factory_node_system)
        .add_system(brownian_motion_system)
        .run();
}

fn brownian_motion_system(mut query: Query<(&Cell, &mut Transform)>) {
    let mut rng = thread_rng();
    for (_, mut transform) in query.iter_mut() {
        transform.translation +=
            Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
    }
}

fn factory_node_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<&mut FactoryNode>,
) {
    for mut factory_node in &mut query.iter_mut() {
        factory_node.timer -= time.delta_seconds();

        if factory_node.timer <= 0.0 {
            factory_node.timer = factory_node.time_to_spawn;

            // TODO: Do not load sprite every time, wtf?
            let sprite_handle = assets.load("placeholder_square.png");

            commands
                .spawn_bundle(SpriteBundle {
                    texture: sprite_handle.clone(),
                    transform: Transform::from_xyz(100.0, 0.0, 0.0),
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Cell);
        }
    }
}
