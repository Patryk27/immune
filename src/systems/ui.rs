use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .add_event::<UiEvent>()
            .add_system(process_events)
            .add_system(draw);
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

fn draw(mut egui: ResMut<EguiContext>, mut state: ResMut<UiState>) {
    if let Some(_) = state.opened_lymph_node {
        let mut open = true;

        egui::Window::new("Lymph Node")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .resizable(true)
            .collapsible(false)
            .open(&mut open)
            .show(egui.ctx_mut(), |ui| {
                let mut input_1 = LymphNodeInput::A;
                let mut input_2 = LymphNodeInput::B;

                egui::Grid::new("lymph-node.recipe").num_columns(5).show(
                    ui,
                    |ui| {
                        ui.vertical(|ui| {
                            for input in LymphNodeInput::all() {
                                ui.horizontal(|ui| {
                                    ui.radio_value(
                                        &mut input_1,
                                        input,
                                        input.label(),
                                    );
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
                            for input in LymphNodeInput::all() {
                                ui.horizontal(|ui| {
                                    ui.radio_value(
                                        &mut input_2,
                                        input,
                                        input.label(),
                                    );
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

        if !open {
            state.opened_lymph_node = None;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LymphNodeInput {
    A,
    B,
    C,
    D,
    Other,
}

impl LymphNodeInput {
    pub fn label(self) -> &'static str {
        match self {
            LymphNodeInput::A => "Protein A",
            LymphNodeInput::B => "Protein B",
            LymphNodeInput::C => "Protein C",
            LymphNodeInput::D => "Protein D",
            LymphNodeInput::Other => "Other lymph node",
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [
            LymphNodeInput::A,
            LymphNodeInput::B,
            LymphNodeInput::C,
            LymphNodeInput::D,
            LymphNodeInput::Other,
        ]
        .into_iter()
    }
}
