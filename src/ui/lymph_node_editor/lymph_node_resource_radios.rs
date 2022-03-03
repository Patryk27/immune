use std::iter;

use bevy_egui::egui::{Response, Ui, Widget};

use super::UiLymphNodeResourceRadio;
use crate::systems::bio::LymphNodeResource;
use crate::ui::UiTextures;

pub struct UiLymphNodeResourceRadios<'a> {
    textures: &'a UiTextures,
    label: &'a str,
    current_value: &'a mut Option<LymphNodeResource>,
}

impl<'a> UiLymphNodeResourceRadios<'a> {
    pub fn new(
        textures: &'a UiTextures,
        label: &'a str,
        current_value: &'a mut Option<LymphNodeResource>,
    ) -> Self {
        Self {
            label,
            textures,
            current_value,
        }
    }
}

impl Widget for UiLymphNodeResourceRadios<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut changed = false;

        let mut response = ui
            .vertical(|ui| {
                ui.label(self.label);
                ui.add_space(3.0);

                let values = iter::once(None)
                    .chain(LymphNodeResource::variants().map(Some));

                for value in values {
                    let response = ui.add(UiLymphNodeResourceRadio::new(
                        self.textures,
                        *self.current_value,
                        value,
                    ));

                    if response.clicked() {
                        *self.current_value = value;
                        changed = true;
                    }
                }
            })
            .response;

        if changed {
            response.mark_changed();
        }

        response
    }
}
