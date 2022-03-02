mod debug_window;
mod lymph_node_editor;
mod radio_image_button;
mod textures;

use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_prototype_debug_lines::DebugLines;

pub(self) use self::lymph_node_editor::*;
pub(self) use self::radio_image_button::*;
pub(self) use self::textures::*;
use crate::compiling::RecompileEvent;
use crate::systems::cell_node::*;
use crate::systems::highlight::SelectorHighlight;
use crate::systems::input::InputState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .insert_resource(UiTextures::default())
            .add_event::<UiEvent>()
            .add_system(process_events)
            .add_system(process_lymph_node_editor)
            // Debug
            .insert_resource(crate::systems::debug::DebugState::default())
            .add_system(debug_window::system);
    }
}

pub enum UiEvent {
    LymphNodeClicked(Entity),
}

#[derive(Default)]
struct UiState {
    lymph_node_editor: Option<UiLymphNodeEditor>,
}

fn process_events(
    assets: Res<AssetServer>,
    mut egui: ResMut<EguiContext>,
    mut textures: ResMut<UiTextures>,
    mut events: EventReader<UiEvent>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<UiState>,
) {
    for event in events.iter() {
        match event {
            UiEvent::LymphNodeClicked(lymph_node) => {
                if let Some(editor) = &mut state.lymph_node_editor {
                    editor.notify(UiLymphNodeEditorEvent::LymphNodeClicked(
                        *lymph_node,
                    ));
                } else {
                    state.lymph_node_editor = Some(UiLymphNodeEditor::new(
                        &assets,
                        &mut egui,
                        &mut textures,
                        *lymph_node,
                    ));
                }
            }
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        if let Some(editor) = &mut state.lymph_node_editor {
            editor.notify(UiLymphNodeEditorEvent::EscapePressed);
        }
    }
}

fn process_lymph_node_editor(
    lines: ResMut<DebugLines>,
    egui: ResMut<EguiContext>,
    textures: Res<UiTextures>,
    input: Res<InputState>,
    mut state: ResMut<UiState>,
    lymph_nodes: Query<(&mut LymphNode, &Transform, &Children, Entity)>,
    recompile_event_tx: EventWriter<RecompileEvent>,
    mut highlights: Query<&mut Visibility, With<SelectorHighlight>>,
) {
    if let Some(editor) = &mut state.lymph_node_editor {
        for (_, _, children, entity) in lymph_nodes.iter() {
            for &child in children.iter() {
                if let Ok(mut highlight_visibility) = highlights.get_mut(child)
                {
                    highlight_visibility.is_visible =
                        entity == editor.lymph_node();
                }
            }
        }

        match editor.process(
            lines,
            egui,
            &textures,
            input.mouse_pos,
            lymph_nodes,
            recompile_event_tx,
        ) {
            UiLymphNodeEditorOutcome::Awaiting => {
                //
            }
            UiLymphNodeEditorOutcome::Completed => {
                state.lymph_node_editor = None;
            }
        }
    } else {
        for (_, _, children, _) in lymph_nodes.iter() {
            for &child in children.iter() {
                if let Ok(mut highlight_visibility) = highlights.get_mut(child)
                {
                    highlight_visibility.is_visible = false;
                }
            }
        }
    }
}
