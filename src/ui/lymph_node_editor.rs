mod lymph_node_input_radio;
mod lymph_node_input_radios;

use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

use self::lymph_node_input_radio::*;
use self::lymph_node_input_radios::*;
use super::*;
use crate::compiling::RecompileEvent;
use crate::systems::cell_node::*;
use crate::theme;

pub struct UiLymphNodeEditor {
    lymph_node: Entity,
}

impl UiLymphNodeEditor {
    pub fn new(
        assets: &AssetServer,
        egui: &mut EguiContext,
        textures: &mut UiTextures,
        lymph_node: Entity,
    ) -> Self {
        for input in LymphNodeInput::variants() {
            textures.load(assets, egui, input.asset_path());
        }

        Self { lymph_node }
    }

    pub fn process(
        &mut self,
        mut egui: ResMut<EguiContext>,
        textures: &UiTextures,
        mut lymph_nodes: Query<(Entity, &mut LymphNode)>,
        mut recompile_event_tx: EventWriter<RecompileEvent>,
    ) -> Result<(), ()> {
        let (_, mut lymph_node) =
            lymph_nodes.get_mut(self.lymph_node).map_err(drop)?;

        let mut keep_opened = true;
        let mut changed = false;

        egui::Window::new("Lymph Node")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut keep_opened)
            .show(egui.ctx_mut(), |ui| {
                egui::Grid::new("lymph-node-editor.recipe").show(ui, |ui| {
                    changed |= ui
                        .add(UiLymphNodeInputRadios::new(
                            textures,
                            &mut lymph_node.lhs,
                            "Input A:",
                        ))
                        .changed();

                    ui.centered_and_justified(|ui| {
                        ui.label("+");
                    });

                    changed |= ui
                        .add(UiLymphNodeInputRadios::new(
                            textures,
                            &mut lymph_node.rhs,
                            "Input B:",
                        ))
                        .changed();
                });

                ui.shrink_width_to_current();

                if lymph_node.output.is_none() {
                    ui.separator();
                    ui.add_space(3.0);
                    ui.colored_label(theme::ui::text_danger_egui(), "[!] This lymph node has invalid configuration and does not produce any cells.");
                }
            });

        if changed {
            recompile_event_tx.send(RecompileEvent);
        }

        if keep_opened {
            Ok(())
        } else {
            Err(())
        }
    }
}
