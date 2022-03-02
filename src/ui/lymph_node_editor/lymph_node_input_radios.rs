use std::iter;

use bevy_egui::egui::{Response, Ui, Widget};

use super::UiLymphNodeInputRadio;
use crate::systems::cell_node::LymphNodeInput;
use crate::ui::UiTextures;

pub struct UiLymphNodeInputRadios<'a> {
    textures: &'a UiTextures,
    label: &'a str,
    selected_value: &'a mut Option<LymphNodeInput>,
    needs_node_picker: &'a mut bool,
}

impl<'a> UiLymphNodeInputRadios<'a> {
    pub fn new(
        textures: &'a UiTextures,
        label: &'a str,
        selected_value: &'a mut Option<LymphNodeInput>,
        needs_node_picker: &'a mut bool,
    ) -> Self {
        Self {
            label,
            textures,
            selected_value,
            needs_node_picker,
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

                let values = iter::once(None)
                    .chain(LymphNodeInput::variants().map(Some));

                for value in values {
                    let response = ui.add(UiLymphNodeInputRadio::new(
                        self.textures,
                        value,
                        *self.selected_value,
                        self.needs_node_picker,
                    ));

                    if response.clicked() {
                        changed = true;
                        *self.selected_value = value;
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
