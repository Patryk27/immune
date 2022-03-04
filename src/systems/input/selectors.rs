use bevy::prelude::*;

use super::{
    Collider, InputState, MousePos, SelectedUnits, SelectedUnitsChanged,
    Selector,
};
use crate::systems::units::Unit;

const HIGHLIGHT_ZOOM_MAX: f32 = 1.1;
const HIGHLIGHT_ZOOM_MIN: f32 = 0.9;
const HIGHLIGHT_ZOOM_SPEED: f32 = 0.01;

pub fn track_selector_hovers(
    mut state: ResMut<InputState>,
    mouse_pos: Res<MousePos>,
    entities: Query<(Entity, &Collider, &Transform, &Children)>,
    mut selectors: Query<&mut Selector>,
) {
    if state.is_dragging {
        // looks kiczowato
        return;
    }

    let new_hovered_entity =
        entities
            .iter()
            .find_map(|(entity, collider, transform, children)| {
                if collider
                    .contains(transform.translation.truncate(), mouse_pos.0)
                {
                    Some((entity, children))
                } else {
                    None
                }
            });

    if new_hovered_entity.map(|(e, _)| e) != state.hovered_entity {
        if let Some(old_hovered_entity) = state.hovered_entity {
            if let Ok((_, _, _, children)) = entities.get(old_hovered_entity) {
                Selector::modify(&mut selectors, children, |selector| {
                    selector.hovered = false;
                });
            }
        }

        if let Some((_, children)) = new_hovered_entity {
            Selector::modify(&mut selectors, children, |selector| {
                selector.hovered = true;
            });
        }

        state.hovered_entity = new_hovered_entity.map(|(e, _)| e);
    }
}

pub fn update_selector_highlights(
    mut selected_units_changes: EventReader<SelectedUnitsChanged>,
    state: Res<SelectedUnits>,
    units: Query<(Entity, &Unit, &Children)>,
    mut selectors: Query<&mut Selector>,
) {
    if selected_units_changes.iter().next().is_none() {
        return;
    }

    for (entity, _, children) in units.iter() {
        for &child in children.iter() {
            if let Ok(mut selector) = selectors.get_mut(child) {
                selector.picked = state.selected_units.contains(&entity);
            }
        }
    }
}

pub fn animate_selectors(
    mut state: ResMut<InputState>,
    mut selectors: Query<(&Selector, &mut Visibility, &mut Transform)>,
) {
    if state.highlight_zoom + HIGHLIGHT_ZOOM_SPEED > HIGHLIGHT_ZOOM_MAX {
        state.highlight_zoom_dir = -1.0
    } else if state.highlight_zoom - HIGHLIGHT_ZOOM_SPEED < HIGHLIGHT_ZOOM_MIN {
        state.highlight_zoom_dir = 1.0
    }

    state.highlight_zoom += HIGHLIGHT_ZOOM_SPEED * state.highlight_zoom_dir;

    for (selector, mut visibility, mut transform) in selectors.iter_mut() {
        visibility.is_visible = selector.hovered || selector.picked;

        if selector.picked {
            transform.scale.x = state.highlight_zoom * Selector::SCALE;
            transform.scale.y = state.highlight_zoom * Selector::SCALE;
        } else {
            transform.scale.x = 1.0 * Selector::SCALE;
            transform.scale.y = 1.0 * Selector::SCALE;
        }
    }
}
