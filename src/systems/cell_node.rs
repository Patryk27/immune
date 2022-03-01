mod antigen;
mod antigen_binder;
mod body;
mod cell;
mod leukocyte;
mod lymph_node;
mod pathogen;
mod protein;

use std::f32::consts::TAU;

use bevy::math::vec3;
use bevy::prelude::*;
use keyframe::functions::EaseInOutCubic;
use keyframe::EasingFunction;
use rand::Rng;

pub use self::antigen::*;
pub use self::antigen_binder::*;
pub use self::body::*;
pub use self::cell::*;
pub use self::leukocyte::*;
pub use self::lymph_node::*;
pub use self::pathogen::*;
pub use self::protein::*;
use super::physics::PHYSICS_SCALE;

pub fn initialize(app: &mut App) {
    app.add_system(progress_lymph_nodes)
        .add_system(animate_progress_bars)
        .add_system(animate_fresh_cells);
}

fn progress_lymph_nodes(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<AssetServer>,
    mut query: Query<(Entity, &mut LymphNode, &Transform)>,
) {
    for (_, mut lymph_node, transform) in &mut query.iter_mut() {
        let output = if let Some(output) = &lymph_node.output {
            *output
        } else {
            continue;
        };

        let lymph_node = &mut *lymph_node;

        lymph_node.production_tt += time.delta_seconds();

        if lymph_node.production_tt >= lymph_node.production_duration {
            lymph_node.production_tt = 0.0;

            match output {
                LymphNodeOutput::Leukocyte(leukocyte) => {
                    let mut rng = rand::thread_rng();

                    let pos = transform.translation.truncate() / PHYSICS_SCALE;

                    let vel = {
                        let angle = Transform::default().with_rotation(
                            Quat::from_axis_angle(
                                Vec3::Z,
                                rng.gen_range(0.0..=TAU),
                            ),
                        );

                        let speed = Transform::default().with_translation(
                            vec3(8.0 * rng.gen_range(1.0..4.0), 0.0, 0.0),
                        );

                        (angle * speed).translation.truncate()
                    };

                    leukocyte.spawn(&mut commands, &assets, pos, vel);
                }
            }
        }
    }
}

fn animate_progress_bars(
    lymph_nodes: Query<&LymphNode>,
    mut progress_bars: Query<
        (&Parent, &mut Transform),
        With<LymphNodeProgressBar>,
    >,
) {
    for (parent, mut transform) in progress_bars.iter_mut() {
        let node = lymph_nodes.get(**parent).unwrap();

        let progress = if node.output.is_some() {
            node.production_tt / node.production_duration
        } else {
            0.0
        };

        *transform = Transform::default()
            .with_translation(vec3(0.0, 0.0, 0.1))
            .with_scale(vec3(progress, 1.0, 1.0));
    }
}

fn animate_fresh_cells(
    mut commands: Commands,
    time: Res<Time>,
    mut cell: Query<(Entity, &mut CellFadeIn, &mut Sprite), With<CellFadeIn>>,
) {
    for (entity, mut tag, mut sprite) in cell.iter_mut() {
        tag.tt += time.delta_seconds() * 2.5;

        sprite.color = {
            let [r, g, b, _] = sprite.color.as_rgba_f32();
            let a = EaseInOutCubic.y(tag.tt.min(1.0) as _) as _;

            Color::rgba(r, g, b, a)
        };

        if tag.tt >= 1.0 {
            commands.entity(entity).remove::<CellFadeIn>();
        }
    }
}
