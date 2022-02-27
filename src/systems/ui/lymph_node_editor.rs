use std::iter;

use bevy::prelude::*;
use bevy_egui::egui::{self, Align2, Sense, Ui};
use bevy_egui::EguiContext;

use super::UiTextures;
use crate::systems::cell_node::*;

pub struct UiLymphNodeEditor {
    lymph_node: Entity,
}

impl UiLymphNodeEditor {
    pub fn new(
        assets: &AssetServer,
        egui: &mut EguiContext,
        textures: &mut UiTextures,
        lymph_node: Entity,
    ) -> Self {
        for variant in LymphNodeInput::variants() {
            textures.load(assets, egui, variant.asset_path());
        }

        Self { lymph_node }
    }

    pub fn process(
        &mut self,
        mut egui: ResMut<EguiContext>,
        textures: &UiTextures,
        mut lymph_nodes: Query<&mut LymphNode>,
    ) -> Result<(), ()> {
        let mut lymph_node =
            lymph_nodes.get_mut(self.lymph_node).map_err(drop)?;

        let mut lhs = UiLymphNodeInput::new(lymph_node.lhs);
        let mut rhs = UiLymphNodeInput::new(lymph_node.rhs);

        let mut open = true;

        egui::Window::new("Lymph Node")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut open)
            .show(egui.ctx_mut(), |ui| {
                egui::Grid::new("lymph-node-editor.recipe")
                    .num_columns(5)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for item in UiLymphNodeInput::variants() {
                                item.show(textures, ui, &mut lhs);
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
                                item.show(textures, ui, &mut rhs);
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
                    });
            });

        lhs.store_to(&mut lymph_node.lhs);
        rhs.store_to(&mut lymph_node.rhs);

        if open {
            Ok(())
        } else {
            Err(())
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

    fn asset_path(&self) -> Option<&'static str> {
        match self {
            Self::Selected(input) => Some(input.asset_path()),
            Self::None => None, // TODO(pwy) cross icon, maybe?
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

    fn show(self, textures: &UiTextures, ui: &mut Ui, selected: &mut Self) {
        ui.horizontal(|ui| {
            ui.add_space(5.0);

            if let Some(asset_path) = self.asset_path() {
                // TODO(pwy) create a separate RadioImage component?
                ui.radio_value(selected, self, "");

                let image =
                    egui::Image::new(textures.get(asset_path), (40.0, 40.0))
                        .sense(Sense::click());

                if ui.add(image).clicked() {
                    *selected = self;
                }
            } else {
                ui.radio_value(selected, self, "(nothing)");
            }

            ui.add_space(5.0);
        });
    }
}
