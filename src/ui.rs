mod debug_window;
mod lymph_node_editor;
mod radio_image_button;
mod textures;

use bevy::prelude::*;
use bevy_egui::EguiContext;

pub(self) use self::lymph_node_editor::*;
pub(self) use self::radio_image_button::*;
pub(self) use self::textures::*;
use crate::systems::cell_node::*;

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
    OpenLymphNodeEditor(Entity),
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
    mut state: ResMut<UiState>,
) {
    for event in events.iter() {
        match event {
            UiEvent::OpenLymphNodeEditor(lymph_node) => {
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

fn process_lymph_node_editor(
    commands: Commands,
    egui: ResMut<EguiContext>,
    textures: Res<UiTextures>,
    mut state: ResMut<UiState>,
    lymph_nodes: Query<(Entity, &mut LymphNode)>,
) {
    if let Some(editor) = &mut state.lymph_node_editor {
        if editor
            .process(commands, egui, &textures, lymph_nodes)
            .is_err()
        {
            // The lymph node must've been destroyed (e.g. map got reloaded)
            state.lymph_node_editor = None;
        }
    }
}
