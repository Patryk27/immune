mod lymph_node_input_radio;
mod lymph_node_input_radios;
mod lymph_node_picker;

use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

use self::lymph_node_input_radio::*;
use self::lymph_node_input_radios::*;
use self::lymph_node_picker::*;
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
        let asset_paths =
            LymphNodeInput::variants().flat_map(|input| input.asset_path());

        for asset_path in asset_paths {
            textures.load(assets, egui, asset_path);
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

        let (mut lymph_node, lymph_node_transform, _, lymph_node_entity) =
            if let Ok(val) = lymph_nodes.get_mut(self.lymph_node) {
                val
            } else {
                return UiLymphNodeEditorOutcome::Completed;
            };

        let mut changed = false;

        if let Some(picker) = &mut self.lymph_node_picker {
            match picker.process(
                lines,
                mouse_pos,
                &mut lymph_node,
                lymph_node_entity,
                *lymph_node_transform,
            ) {
                UiLymphNodePickerOutcome::Awaiting => {
                    return UiLymphNodeEditorOutcome::Awaiting;
                }

                UiLymphNodePickerOutcome::Completed => {
                    self.lymph_node_picker = None;
                    changed |= true;
                }
            }
        }

        let mut keep_opened = true;

        egui::Window::new("Lymph Node")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut keep_opened)
            .show(egui.ctx_mut(), |ui| {
                egui::Grid::new("lymph-node-editor.recipe").show(ui, |ui| {
                    let mut needs_node_picker = false;

                    changed |= ui
                        .add(UiLymphNodeInputRadios::new(
                            textures,
                            "Input A:",
                            &mut lymph_node.lhs,
                            &mut needs_node_picker,
                        ))
                        .changed();

                    if needs_node_picker {
                        self.lymph_node_picker = Some(UiLymphNodePicker::lhs());
                    }

                    // ---

                    ui.centered_and_justified(|ui| {
                        ui.label("+");
                    });

                    // ---

                    let mut needs_node_picker = false;

                    changed |= ui
                        .add(UiLymphNodeInputRadios::new(
                            textures,
                            "Input B:",
                            &mut lymph_node.rhs,
                            &mut needs_node_picker,
                        ))
                        .changed();

                    if needs_node_picker {
                        self.lymph_node_picker = Some(UiLymphNodePicker::rhs());
                    }
                });

                ui.shrink_width_to_current();

                // ---

                ui.separator();
                ui.add_space(3.0);
                changed |= ui.checkbox(&mut lymph_node.state.paused, "Paused").changed();

                // ---

                if lymph_node.output.is_none() {
                    ui.add_space(3.0);
                    ui.separator();
                    ui.add_space(3.0);
                    ui.colored_label(theme::ui::text_danger_egui(), "[!] This lymph node has invalid configuration and does not produce any cells.");
                }

                if lymph_node.state.awaiting_resources {
                    ui.add_space(3.0);
                    ui.separator();
                    ui.add_space(3.0);
                    ui.colored_label(theme::ui::text_danger_egui(), "[!] Some of this lymph node's inputs are paused.");
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
