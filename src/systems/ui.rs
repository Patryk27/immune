use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

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

pub fn draw(mut egui: ResMut<EguiContext>) {
    egui::Window::new("Lymph Node")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .resizable(true)
        .collapsible(false)
        .show(egui.ctx_mut(), |ui| {
            let mut input_1 = LymphNodeInput::A;
            let mut input_2 = LymphNodeInput::B;

            ui.heading("Product:");
            ui.add_space(10.0);

            egui::Grid::new("lymph-node.recipe")
                .min_col_width(80.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for input in LymphNodeInput::all() {
                            ui.radio_value(&mut input_1, input, input.label());
                        }
                    });

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("+");
                        });
                    });

                    ui.vertical(|ui| {
                        for input in LymphNodeInput::all() {
                            ui.radio_value(&mut input_2, input, input.label());
                        }
                    });

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("=");
                        });
                    });

                    ui.vertical(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("something");
                        });
                    });
                });
        });
}
