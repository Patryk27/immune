use bevy_egui::egui::{Response, Ui, Widget};

use super::UiLymphNodeInputRadio;
use crate::systems::cell_node::LymphNodeInput;
use crate::ui::UiTextures;

pub struct UiLymphNodeInputRadios<'a> {
    textures: &'a UiTextures,
    selected_value: &'a mut Option<LymphNodeInput>,
}

impl<'a> UiLymphNodeInputRadios<'a> {
    pub fn new(
        textures: &'a UiTextures,
        selected_value: &'a mut Option<LymphNodeInput>,
    ) -> Self {
        Self {
            textures,
            selected_value,
        }
    }
}

impl Widget for UiLymphNodeInputRadios<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let radios = UiLymphNodeInputRadio::variants(
                self.textures,
                *self.selected_value,
            );

            for radio in radios {
                if ui.add(radio).clicked() {
                    *self.selected_value = radio.value();
                }
            }
        })
        .response
    }
}
