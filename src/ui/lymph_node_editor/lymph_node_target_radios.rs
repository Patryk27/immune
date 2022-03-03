use bevy_egui::egui::{Response, Ui, Widget};

use crate::systems::bio::LymphNodeTarget;

pub struct UiLymphNodeTargetRadios<'a> {
    current_value: &'a mut LymphNodeTarget,
    requests_node_picker: &'a mut bool,
}

impl<'a> UiLymphNodeTargetRadios<'a> {
    pub fn new(
        current_value: &'a mut LymphNodeTarget,
        requests_node_picker: &'a mut bool,
    ) -> Self {
        Self {
            current_value,
            requests_node_picker,
        }
    }
}

impl Widget for UiLymphNodeTargetRadios<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut changed = false;

        let mut response = ui
            .vertical(|ui| {
                ui.vertical(|ui| {
                    ui.label("Spawn target:");
                });

                ui.vertical(|ui| {
                    let response = ui.radio(
                        matches!(self.current_value, LymphNodeTarget::Outside),
                        "Spawn products outside",
                    );

                    if response.clicked() {
                        *self.current_value = LymphNodeTarget::Outside;
                        changed = true;
                    }
                });

                ui.vertical(|ui| {
                    let response = ui.radio(
                        matches!(
                            self.current_value,
                            LymphNodeTarget::LymphNode(_)
                        ),
                        "Send products to another node",
                    );

                    if response.clicked() {
                        *self.requests_node_picker = true;
                    }

                    ui.horizontal(|ui| {
                        ui.add_space(
                            ui.spacing().icon_width
                                + ui.spacing().button_padding.x,
                        );

                        if ui.button("Pick node").clicked() {
                            *self.requests_node_picker = true;
                        }
                    });
                });
            })
            .response;

        if changed {
            response.mark_changed();
        }

        response
    }
}
