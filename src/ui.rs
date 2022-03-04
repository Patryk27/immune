mod debug_windows;
mod lymph_node_editor;
mod poll;
mod radio_image_button;
mod textures;
mod tutorial;

use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_prototype_debug_lines::DebugLines;

pub(self) use self::lymph_node_editor::*;
pub(self) use self::poll::*;
pub(self) use self::radio_image_button::*;
pub(self) use self::textures::*;
use crate::compiling::RecompileEvent;
use crate::systems::bio::*;
use crate::systems::input::{MousePos, Selector};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .insert_resource(UiTextures::default())
            .add_startup_system(tutorial::load_assets)
            .add_event::<UiEvent>()
            .add_system(process_events)
            .add_system(process_lymph_node_editor)
            .add_system(tutorial::system);

        debug_windows::initialize(app);
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
    mut selectors: Query<&mut Selector>,
    lymph_nodes: Query<&Children, With<LymphNode>>,
) {
    for event in events.iter() {
        match event {
            UiEvent::LymphNodeClicked(lymph_node) => {
                if let Some(editor) = &mut state.lymph_node_editor {
                    editor.on_lymph_node_clicked(
                        &mut selectors,
                        &lymph_nodes,
                        *lymph_node,
                    );
                } else {
                    state.lymph_node_editor = Some(UiLymphNodeEditor::new(
                        &assets,
                        &mut egui,
                        &mut textures,
                        &mut selectors,
                        &lymph_nodes,
                        *lymph_node,
                    ));
                }
            }
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        if let Some(editor) = &mut state.lymph_node_editor {
            editor.on_escape_pressed();
        }
    }
}

fn process_lymph_node_editor(
    lines: ResMut<DebugLines>,
    egui: ResMut<EguiContext>,
    textures: Res<UiTextures>,
    mouse_pos: Res<MousePos>,
    mut state: ResMut<UiState>,
    mut lymph_nodes: Query<(&mut LymphNode, &Transform, &Children, Entity)>,
    recompile_event_tx: EventWriter<RecompileEvent>,
    mut selectors: Query<&mut Selector>,
    mut events: EventWriter<UiEvent>,
) {
    if let Some(editor) = &mut state.lymph_node_editor {
        match editor.process(
            lines,
            egui,
            &textures,
            mouse_pos.0,
            &mut lymph_nodes,
            recompile_event_tx,
        ) {
            Poll::Pending => {
                //
            }

            Poll::Ready(node) => {
                if let Ok((_, _, children, _)) =
                    lymph_nodes.get(editor.lymph_node())
                {
                    Selector::modify(&mut selectors, children, |selector| {
                        selector.picked = false;
                    });
                }

                state.lymph_node_editor = None;

                if let Some(node) = node {
                    events.send(UiEvent::LymphNodeClicked(node));
                }
            }
        }
    }
}
