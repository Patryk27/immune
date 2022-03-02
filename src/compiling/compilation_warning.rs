use bevy::math::vec3;
use bevy::prelude::*;

use crate::theme;

#[derive(Component, Clone, Debug, Default)]
pub struct CompilationWarning {
    pub(super) tt: f32,
}

impl CompilationWarning {
    pub fn spawn(assets: &AssetServer, entity: &mut ChildBuilder) {
        let transform = Transform::default()
            .with_translation(vec3(
                0.0,
                25.0,
                theme::z_index::LYMPH_NODE_COMPILATION_WARNING
                    - theme::z_index::LYMPH_NODE,
            ))
            .with_scale(vec3(0.8, 0.8, 1.0));

        entity
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(255, 190, 17),
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
        warn.tt += 5.0 * time.delta_seconds();
        warn_sprite.color.set_a(warn.tt.sin().abs());
    }
}
