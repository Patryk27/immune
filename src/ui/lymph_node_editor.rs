mod lymph_node_picker;
mod lymph_node_resource_radio;
mod lymph_node_resource_radios;
mod lymph_node_target_radios;

use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

use self::lymph_node_picker::*;
use self::lymph_node_resource_radio::*;
use self::lymph_node_resource_radios::*;
use self::lymph_node_target_radios::UiLymphNodeTargetRadios;
use super::*;
use crate::compiling::RecompileEvent;
use crate::systems::bio::*;
use crate::theme;

pub struct UiLymphNodeEditor {
    alive: bool,
    lymph_node: Entity,
    lymph_node_picker: Option<UiLymphNodePicker>,
}

pub enum UiLymphNodeEditorOutcome {
    Awaiting,
    Completed,
}

impl UiLymphNodeEditor {
    pub fn new(
        assets: &AssetServer,
        egui: &mut EguiContext,
        textures: &mut UiTextures,
        selectors: &mut Query<&mut Selector>,
        lymph_nodes: &Query<&Children, With<LymphNode>>,
        lymph_node: Entity,
    ) -> Self {
        for variant in LymphNodeResource::variants() {
            textures.load(assets, egui, variant.asset_path());
        }

        if let Ok(children) = lymph_nodes.get(lymph_node) {
            Selector::modify(selectors, children, |selector| {
                selector.picked = true;
            });
        }

        Self {
            alive: true,
            lymph_node,
            lymph_node_picker: None,
        }
    }

    pub fn lymph_node(&self) -> Entity {
        self.lymph_node
    }

    pub fn process(
        &mut self,
        lines: ResMut<DebugLines>,
        mut egui: ResMut<EguiContext>,
        textures: &UiTextures,
        mouse_pos: Vec2,
        lymph_nodes: &mut Query<(
            &mut LymphNode,
            &Transform,
            &Children,
            Entity,
        )>,
        mut recompile_event_tx: EventWriter<RecompileEvent>,
    ) -> UiLymphNodeEditorOutcome {
        if !self.alive {
            return UiLymphNodeEditorOutcome::Completed;
        }

        let mut changed = false;

        if let Some(picker) = &mut self.lymph_node_picker {
            match picker.process(lines, mouse_pos, lymph_nodes, self.lymph_node)
            {
                UiLymphNodePickerOutcome::Awaiting => {
                    return UiLymphNodeEditorOutcome::Awaiting;
                }

                UiLymphNodePickerOutcome::Completed => {
                    self.lymph_node_picker = None;
                    changed |= true;
                }
            }
        }

        let (mut lymph_node, _, _, _) =
            if let Ok(val) = lymph_nodes.get_mut(self.lymph_node) {
                val
            } else {
                return UiLymphNodeEditorOutcome::Completed;
            };

        let mut keep_opened = true;

        egui::Window::new("Lymph Node")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut keep_opened)
            .show(egui.ctx_mut(), |ui| {
                egui::Grid::new("lymph-node-editor.recipe")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    let response =
                                        ui.add(UiLymphNodeResourceRadios::new(
                                            textures,
                                            "Resource:",
                                            &mut lymph_node.resource,
                                        ));

                                    changed |= response.changed();
                                });

                                ui.add_space(6.0);
                                ui.separator();
                            });
                        });

                        ui.vertical(|ui| {
                            let mut requests_node_picker = false;

                            let response =
                                ui.add(UiLymphNodeTargetRadios::new(
                                    &mut lymph_node.target,
                                    &mut requests_node_picker,
                                ));

                            changed |= response.changed();

                            if requests_node_picker {
                                self.lymph_node_picker =
                                    Some(UiLymphNodePicker::new());
                            }
                        });
                    });

                ui.shrink_width_to_current();

                ui.vertical(|ui| {
                    ui.separator();
                    ui.add_space(3.0);

                    let response =
                        ui.checkbox(&mut lymph_node.state.is_paused, "Paused");

                    changed |= response.changed();
                });

                if let Some(warning) = lymph_node.warning {
                    ui.add_space(3.0);
                    ui.separator();
                    ui.add_space(3.0);

                    ui.colored_label(
                        theme::ui::text_danger_egui(),
                        warning.description(),
                    );
                }
            });

        if changed {
            recompile_event_tx.send(RecompileEvent);
        }

        if keep_opened {
            UiLymphNodeEditorOutcome::Awaiting
        } else {
            UiLymphNodeEditorOutcome::Completed
        }
    }

    pub fn on_escape_pressed(&mut self) {
        if let Some(picker) = &mut self.lymph_node_picker {
            picker.on_escape_pressed();
        } else {
            self.alive = false;
        }
    }

    pub fn on_lymph_node_clicked(
        &mut self,
        selectors: &mut Query<&mut Selector>,
        lymph_nodes: &Query<&Children, With<LymphNode>>,
        node: Entity,
    ) {
        if let Some(picker) = &mut self.lymph_node_picker {
            picker.on_lymph_node_clicked(node);
        } else {
            if let Ok(children) = lymph_nodes.get(self.lymph_node) {
                Selector::modify(selectors, children, |selector| {
                    selector.picked = false;
                });
            }

            self.lymph_node = node;

            if let Ok(children) = lymph_nodes.get(self.lymph_node) {
                Selector::modify(selectors, children, |selector| {
                    selector.picked = true;
                });
            }
        }
    }
}
