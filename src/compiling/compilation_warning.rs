use bevy::math::vec3;
use bevy::prelude::*;

use crate::z_index;

#[derive(Component, Clone, Debug, Default)]
pub struct CompilationWarning {
    pub(super) tt: f32,
}

impl CompilationWarning {
    pub fn spawn(assets: &AssetServer, entity: &mut ChildBuilder) {
        let transform = Transform::default()
            .with_translation(vec3(
                0.0,
                0.0,
                z_index::LYMPH_NODE_COMPILATION_WARNING - z_index::LYMPH_NODE,
            ))
            .with_scale(vec3(0.8, 0.8, 1.0));

        entity
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(255, 0, 0),
                    ..Default::default()
                },
                transform,
                texture: assets.load("warning.png"),
                ..Default::default()
            })
            .insert(Self::default());
    }
}

pub(super) fn blink(
    time: Res<Time>,
    mut warnings: Query<(&mut CompilationWarning, &mut Sprite)>,
) {
    for (mut warn, mut warn_sprite) in warnings.iter_mut() {
        warn_sprite.color =
            Color::rgba_linear(1.0, 0.0, 0.0, warn.tt.sin().abs());

        warn.tt += 5.0 * time.delta_seconds();
    }
}
