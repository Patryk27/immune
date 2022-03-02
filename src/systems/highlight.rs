use bevy::prelude::*;
use bevy_egui::EguiContext;

use crate::systems::input::InputState;
use super::units::Unit;


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

pub struct HighlightPlugin;

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighlightRes::default())
            .add_system(highlight_selected_units)
            .add_system(animate_highlight_selection);
    }
}

pub const ZOOM_MAX: f32 = 1.1;
pub const ZOOM_MIN: f32 = 0.9;
pub const ZOOM_SPEED: f32 = 0.01;
pub const ROTATION_SPEED: f32 = 0.1;

pub struct HighlightRes {
    zoom: f32,
    zoom_dir: f32,
}

impl Default for HighlightRes {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            zoom_dir: 1.0,
        }
    }
}

fn highlight_selected_units(
    mut egui: ResMut<EguiContext>,
    state: ResMut<InputState>,
    units: Query<(Entity, &Unit, &Children)>,
    mut highlights: Query<
        (Entity, &Parent, &mut Visibility),
        With<SelectorHighlight>,
    >,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        return;
    }

    // Has to be greater than max number of children of Unit to allocate only once
    let unit_children = 10;
    let capacity = state.selected_units.len() * unit_children;
    let mut selected_children: Vec<Entity> = Vec::with_capacity(capacity);

    for selected_entity in state.selected_units.iter() {
        let (_, _, children) = units.get(*selected_entity).unwrap();

        selected_children.extend(children.iter());
    }

    for (entity, parent, mut visibility) in highlights.iter_mut() {
        let is_parent_unit = units
            .iter()
            .any(|(unit_entity, _, _)| unit_entity == **parent);

        if !is_parent_unit {
            // We don't want to manage e.g. lymph nodes' selectors
            continue;
        }

        visibility.is_visible = selected_children.contains(&entity);
    }
}

fn animate_highlight_selection(
    mut highlight_state: ResMut<HighlightRes>,
    mut highlights: Query<(&SelectorHighlight, &Visibility, &mut Transform)>,
) {

    if highlight_state.zoom + ZOOM_SPEED > ZOOM_MAX {
        highlight_state.zoom_dir = -1.0
    } else if highlight_state.zoom - ZOOM_SPEED < ZOOM_MIN {
        highlight_state.zoom_dir = 1.0
    }

    highlight_state.zoom += ZOOM_SPEED * highlight_state.zoom_dir;

    for (_, visibility, mut transform) in highlights.iter_mut() {
        if visibility.is_visible {
            transform.scale.x = highlight_state.zoom;
            transform.scale.y = highlight_state.zoom;
        } else {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        }
    }

}
