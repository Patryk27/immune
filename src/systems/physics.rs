use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn initialize(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup);
}

pub fn setup(
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = [0.0, 0.0].into();
    rapier_config.scale = 1.0;
}
