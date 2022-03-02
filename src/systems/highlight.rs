use bevy::prelude::*;

pub struct Selector;

impl Selector {
    pub fn spawn(
        assets: &AssetServer,
        entity: &mut ChildBuilder,
        size: f32,
        color: Color,
    ) {
        let texture = assets.load("selector.png");
        let arrows = vec![
            (false, false, -1.0, 1.0),
            (true, false, 1.0, 1.0),
            (false, true, -1.0, -1.0),
            (true, true, 1.0, -1.0),
        ];

        for (flip_x, flip_y, mul_x, mul_y) in arrows {
            let transform =
                Transform::from_xyz(size * mul_x, size * mul_y, 0.0);

            entity
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        flip_x,
                        flip_y,
                        ..Default::default()
                    },
                    texture: texture.clone(),
                    transform,
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SelectorHighlight);
        }
    }
}

#[derive(Component)]
pub struct SelectorHighlight;
