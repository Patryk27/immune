mod lymph_node_editor;
mod textures;

use bevy::prelude::*;
use bevy_egui::EguiContext;

use self::lymph_node_editor::*;
use self::textures::*;
use super::cell_node::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .insert_resource(UiTextures::default())
            .add_event::<UiEvent>()
            .add_system(process_events)
            .add_system(process_lymph_node_editor);
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
    egui: ResMut<EguiContext>,
    textures: Res<UiTextures>,
    mut state: ResMut<UiState>,
    lymph_nodes: Query<&mut LymphNode>,
) {
    if let Some(editor) = &mut state.lymph_node_editor {
        if editor.process(egui, &textures, lymph_nodes).is_err() {
            // The lymph node must've been destroyed (e.g. map got reloaded)
            state.lymph_node_editor = None;
        }
    }
}
