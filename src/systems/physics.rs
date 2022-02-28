use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PHYSICS_SCALE: f32 = 100.0;

pub fn initialize(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup);
}

pub fn setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = [0.0, 0.0].into();
    rapier_config.scale = PHYSICS_SCALE;
}

pub fn pixel_to_world(vec: Vec3) -> Vector<Real> {
    (vec / PHYSICS_SCALE).truncate().to_array().into()
}

pub fn world_to_pixel(vec: Vector<Real>) -> Vec3 {
    Vec2::from(vec.data.0[0]).extend(0.0) * PHYSICS_SCALE
}
