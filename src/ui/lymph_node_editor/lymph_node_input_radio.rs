use bevy_egui::egui::{vec2, Response, Ui, Widget};

use crate::systems::cell_node::{Antigen, Leukocyte, LymphNodeInput, Protein};
use crate::theme;
use crate::ui::{UiRadioImageButton, UiTextures};

pub struct UiLymphNodeInputRadio<'a> {
    textures: &'a UiTextures,
    current_value: Option<LymphNodeInput>,
    selected_value: Option<LymphNodeInput>,
    needs_node_picker: &'a mut bool,
}

impl<'a> UiLymphNodeInputRadio<'a> {
    pub fn new(
        textures: &'a UiTextures,
        current_value: Option<LymphNodeInput>,
        selected_value: Option<LymphNodeInput>,
        needs_node_picker: &'a mut bool,
    ) -> Self {
        Self {
            textures,
            current_value,
            selected_value,
            needs_node_picker,
        }
    }

    fn checked(&self) -> bool {
        if self.current_value == self.selected_value {
            return true;
        }

        if let (
            Some(LymphNodeInput::External(_)),
            Some(LymphNodeInput::External(_)),
        ) = (self.current_value, self.selected_value)
        {
            return true;
        }

        false
    }
}

impl Widget for UiLymphNodeInputRadio<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let checked = self.checked();

        ui.horizontal(|ui| match self.current_value {
            Some(
                value @ (LymphNodeInput::Body(_)
                | LymphNodeInput::Binder(_)
                | LymphNodeInput::Protein(_)),
            ) => {
                let size = match value {
                    LymphNodeInput::Binder(_) => vec2(36.0, 36.0),
                    LymphNodeInput::Protein(_) => vec2(26.0, 26.0),
                    _ => vec2(50.0, 50.0),
                };

                let tint = match value {
                    LymphNodeInput::Body(_) => {
                        theme::to_egui(Leukocyte::color(255))
                    }
                    LymphNodeInput::Binder(_) => theme::to_egui(
                        Antigen::color(Leukocyte::color(255), 255),
                    ),
                    LymphNodeInput::Protein(_) => {
                        theme::to_egui(Protein::color())
                    }
                    _ => {
                        unreachable!()
                    }
                };

                let padding = vec2(50.0 - size.x, 5.0);
                let asset_path = value.asset_path().unwrap();

                ui.add(
                    UiRadioImageButton::new(
                        checked,
                        self.textures.get(asset_path),
                        size,
                    )
                    .with_image_padding(padding)
                    .with_image_tint(tint),
                )
            }

            Some(LymphNodeInput::External(node)) => {
                let radio = ui.radio(checked, "  Another node");

                let button =
                    ui.button(if node.is_some() { "Change" } else { "Pick" });

                if button.clicked() {
                    *self.needs_node_picker = true;
                }

                radio
            }

            None => ui.radio(checked, "  Nothing"),
        })
        .inner
    }
}
