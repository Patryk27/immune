use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(parallax)
        .add_system(sway);
}

const MAX_PHASE: f32 = 20.0;

const SWAY_X_MAGNITUDE: f32 = 45.0;
const SWAY_Y_MAGNITUDE: f32 = 35.0;
const SWAY_X_FREQUENCY: f32 = 0.6;
const SWAY_Y_FREQUENCY: f32 = 0.3;

#[derive(Component)]
struct BackgroundItem {
    sway: Vec3,
    offset: Vec3,
    parallax_layer: usize,
    phase: f32,
}

struct BackgroundLayer {
    num: usize,
    scale: f32,
    z: f32,
    parallax_layer: usize,
    spread: f32,
}

const LAYERS: &[BackgroundLayer] = &[
    BackgroundLayer {
        num: 4096,
        scale: 0.2,
        z: 0.2,
        parallax_layer: 6,
        spread: 10_000.0,
    },
    BackgroundLayer {
        num: 512,
        scale: 0.5,
        z: 0.1,
        parallax_layer: 8,
        spread: 10_000.0,
    },
];

fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let blur_circle = assets.load("blur.png");
    let blur_hexagon = assets.load("blur.hexagon.png");
    let images = [blur_circle, blur_hexagon];

    for layer in LAYERS {
        for _ in 0..layer.num {
            spawn(&mut commands, &images, layer);
        }
    }
}

fn spawn(
    commands: &mut Commands,
    images: &[Handle<Image>],
    layer: &BackgroundLayer,
) {
    let parallax_layer = layer.parallax_layer;
    let scale = layer.scale;
    let z = layer.z;

    let mut rng = thread_rng();

    let mut offset = Vec3::ZERO;
    offset.x = (rng.gen::<f32>() * 2.0 - 1.0) * layer.spread;
    offset.y = (rng.gen::<f32>() * 2.0 - 1.0) * layer.spread;
    offset.z = z;

    let phase = rng.gen::<f32>() * MAX_PHASE;

    let mut transform = Transform::from_scale(Vec3::ONE * scale);
    transform.rotation =
        Quat::from_rotation_z(rng.gen::<f32>() * std::f32::consts::PI * 2.0);

    let img_idx = rng.gen::<usize>() % images.len();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: images[img_idx].clone(),
            transform,
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundItem {
            sway: Vec3::new(0.0, 0.0, 0.0),
            parallax_layer,
            offset,
            phase,
        });
}

fn parallax(
    camera: Query<&Transform, With<Camera>>,
    mut bg_items: Query<(&mut Transform, &BackgroundItem), Without<Camera>>,
) {
    let camera = camera.single();

    let mut pos = camera.translation;
    pos.z = 0.0;

    for (mut transform, item) in bg_items.iter_mut() {
        transform.translation =
            item.offset + item.sway + pos / (item.parallax_layer as f32);
    }
}

fn sway(time: Res<Time>, mut bg_items: Query<&mut BackgroundItem>) {
    let t = time.seconds_since_startup() as f32;

    for mut item in bg_items.iter_mut() {
        item.sway.x =
            SWAY_X_MAGNITUDE * f32::sin(item.phase + t * SWAY_X_FREQUENCY);
        item.sway.y =
            SWAY_Y_MAGNITUDE * f32::cos(item.phase + t * SWAY_Y_FREQUENCY);
    }
}
