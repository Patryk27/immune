use bevy::prelude::*;
use instant::Instant;

use super::{GameState, LevelVm};
use crate::level::Level;
use crate::systems::bio::Pathogen;

const FONT_SIZE: f32 = 30.0;
const TOP_MARGIN: f32 = 10.0;
const TEXT_Z_OFFSET: f32 = 10.0;

#[derive(Component)]
struct ProgressText {
    offset: Vec3,
}

#[derive(Component)]
struct WaveText;

#[derive(Component)]
struct NumberOfVirusesText;

pub fn initialize(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(position_text)
        .add_system(update_wave_text)
        .add_system(update_number_of_viruses_text);
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fat-pixels.regular.ttf");

    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("Next wave in"),
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                0.0,
                TEXT_Z_OFFSET,
            )),
            ..Default::default()
        })
        .insert(WaveText)
        .insert(ProgressText {
            offset: Vec3::Y * -30.0,
        });

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("Next wave in"),
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                60.0,
                TEXT_Z_OFFSET,
            )),
            ..Default::default()
        })
        .insert(NumberOfVirusesText)
        .insert(ProgressText { offset: Vec3::ZERO });
}

fn position_text(
    camera: Query<
        (&Camera, &Transform, &OrthographicProjection),
        Without<ProgressText>,
    >,
    mut text: Query<(&mut Transform, &ProgressText), With<ProgressText>>,
) {
    let (_, camera, ortho) = camera.single();

    for (mut transform, text) in text.iter_mut() {
        transform.translation = camera.translation
            + text.offset * ortho.scale
            + Vec3::Y * (ortho.top - FONT_SIZE - TOP_MARGIN) * ortho.scale;

        transform.translation.z = TEXT_Z_OFFSET;
        transform.scale = Vec3::ONE * ortho.scale;
    }
}

fn update_wave_text(
    state: Res<GameState>,
    level: Res<Level>,
    mut query: Query<(&mut Text, &WaveText)>,
) {
    let (mut text, _) = query.single_mut();

    if state.game_over {
        text.sections[0].value = "Game over".into();
        return;
    }

    text.sections[0].value = String::default();

    match state.vm {
        LevelVm::AwaitingStart { at } => {
            if let Some(d) = at.checked_duration_since(Instant::now()) {
                text.sections[0].value =
                    format!("Game begins in {} seconds", d.as_secs());
            }
        }

        LevelVm::AwaitingWaveStart { at } => {
            if let Some(d) = at.checked_duration_since(Instant::now()) {
                text.sections[0].value = format!(
                    "Wave {} starts in {} seconds",
                    level.wave_idx,
                    d.as_secs(),
                );
            }
        }

        _ => (),
    }
}

fn update_number_of_viruses_text(
    viruses: Query<&Pathogen>,
    mut query: Query<(&mut Text, &NumberOfVirusesText)>,
) {
    let (mut text, _) = query.single_mut();

    // TODO(dzejkop): Cache this value?
    let num_of_viruses = viruses.iter().count();

    text.sections[0].value = format!("{} viruses remaining", num_of_viruses);
}
