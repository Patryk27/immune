use std::iter;

use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

use super::cell_node::{LymphNode, LymphNodeInput};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .add_event::<UiEvent>()
            .add_system(process_events)
            .add_system(draw_opened_lymph_node);
    }
}

pub enum UiEvent {
    OpenLymphNode(Entity),
}

// ---

#[derive(Default)]
struct UiState {
    opened_lymph_node: Option<Entity>,
}

fn process_events(
    mut state: ResMut<UiState>,
    mut events: EventReader<UiEvent>,
) {
    for event in events.iter() {
        match event {
            UiEvent::OpenLymphNode(node) => {
                state.opened_lymph_node = Some(*node);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UiLymphNodeInput {
    Selected(LymphNodeInput),
    None,
}

impl UiLymphNodeInput {
    fn new(input: Option<LymphNodeInput>) -> Self {
        match input {
            Some(input) => Self::Selected(input),
            None => Self::None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Selected(input) => input.asset_path(),
            Self::None => "(nothing)",
        }
    }

    fn store_to(self, input: &mut Option<LymphNodeInput>) {
        match self {
            UiLymphNodeInput::Selected(new_input) => {
                *input = Some(new_input);
            }
            UiLymphNodeInput::None => {
                *input = None;
            }
        }
    }

    fn variants() -> impl Iterator<Item = Self> {
        iter::once(Self::None)
            .chain(LymphNodeInput::variants().map(Self::Selected))
    }
}

fn draw_opened_lymph_node(
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<UiState>,
    mut lymph_nodes: Query<&mut LymphNode>,
) {
    let lymph_node = if let Some(lymph_node) = state.opened_lymph_node {
        lymph_node
    } else {
        return;
    };

    let mut lymph_node = if let Ok(lymph_node) = lymph_nodes.get_mut(lymph_node)
    {
        lymph_node
    } else {
        // Node must've been destroyed (e.g. map was reloaded)
        state.opened_lymph_node = None;
        return;
    };

    let mut lhs = UiLymphNodeInput::new(lymph_node.lhs);
    let mut rhs = UiLymphNodeInput::new(lymph_node.rhs);

    let mut open = true;

    egui::Window::new("Lymph Node")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .resizable(false)
        .collapsible(false)
        .open(&mut open)
        .show(egui.ctx_mut(), |ui| {
            egui::Grid::new("lymph-node.recipe").num_columns(5).show(
                ui,
                |ui| {
                    ui.vertical(|ui| {
                        for item in UiLymphNodeInput::variants() {
                            ui.horizontal(|ui| {
                                ui.radio_value(&mut lhs, item, item.label());
                            });
                        }
                    });

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("+");
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        for item in UiLymphNodeInput::variants() {
                            ui.horizontal(|ui| {
                                ui.radio_value(&mut rhs, item, item.label());
                            });
                        }
                    });

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("=");
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("something");
                            });
                        });
                    });
                },
            );
        });

    lhs.store_to(&mut lymph_node.lhs);
    rhs.store_to(&mut lymph_node.rhs);

    if !open {
        state.opened_lymph_node = None;
    }
}
