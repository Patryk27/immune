use bevy_egui::egui::{vec2, Response, Ui, Widget};

use crate::systems::bio::{Antigen, Leukocyte, LymphNodeResource, Protein};
use crate::theme;
use crate::ui::{UiRadioImageButton, UiTextures};

pub struct UiLymphNodeResourceRadio<'a> {
    textures: &'a UiTextures,
    current_value: Option<LymphNodeResource>,
    selected_value: Option<LymphNodeResource>,
}

impl<'a> UiLymphNodeResourceRadio<'a> {
    pub fn new(
        textures: &'a UiTextures,
        current_value: Option<LymphNodeResource>,
        selected_value: Option<LymphNodeResource>,
    ) -> Self {
        Self {
            textures,
            current_value,
            selected_value,
        }
    }

    fn checked(&self) -> bool {
        self.current_value == self.selected_value
    }
}

impl Widget for UiLymphNodeResourceRadio<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let checked = self.checked();

        ui.horizontal(|ui| match self.selected_value {
            Some(value) => {
                let size = match value {
                    LymphNodeResource::Antigen(_) => vec2(36.0, 36.0),
                    LymphNodeResource::Protein(_) => vec2(26.0, 26.0),
                    _ => vec2(50.0, 50.0),
                };

                let tint = match value {
                    LymphNodeResource::Antigen(_) => theme::to_egui(
                        Antigen::color(Leukocyte::color(255), 255),
                    ),
                    LymphNodeResource::Body(_) => {
                        theme::to_egui(Leukocyte::color(255))
                    }
                    LymphNodeResource::Protein(_) => {
                        theme::to_egui(Protein::color())
                    }
                };

                let padding = vec2(50.0 - size.x, 5.0);
                let asset_path = value.asset_path();

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

            None => ui.radio(checked, "  Nothing"),
        })
        .inner
    }
}
