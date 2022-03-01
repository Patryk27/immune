use bevy_egui::egui::{Response, Ui, Widget};

use super::UiLymphNodeInputRadio;
use crate::systems::cell_node::LymphNodeInput;
use crate::ui::UiTextures;

pub struct UiLymphNodeInputRadios<'a> {
    textures: &'a UiTextures,
    selected_value: &'a mut Option<LymphNodeInput>,
    label: &'a str,
}

impl<'a> UiLymphNodeInputRadios<'a> {
    pub fn new(
        textures: &'a UiTextures,
        selected_value: &'a mut Option<LymphNodeInput>,
        label: &'a str,
    ) -> Self {
        Self {
            textures,
            selected_value,
            label,
        }
    }
}

impl Widget for UiLymphNodeInputRadios<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut changed = false;

        let mut response = ui
            .vertical(|ui| {
                ui.label(self.label);
                ui.add_space(3.0);

                let radios = UiLymphNodeInputRadio::variants(
                    self.textures,
                    *self.selected_value,
                );

                for radio in radios {
                    if ui.add(radio).clicked() {
                        *self.selected_value = radio.value();
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
