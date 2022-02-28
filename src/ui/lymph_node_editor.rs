mod lymph_node_input_radio;
mod lymph_node_input_radios;

use bevy::prelude::*;
use bevy_egui::egui::{self, Align2};
use bevy_egui::EguiContext;

use self::lymph_node_input_radio::*;
use self::lymph_node_input_radios::*;
use super::*;
use crate::compiling::NeedsRecompiling;
use crate::systems::cell_node::*;

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
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        textures: &UiTextures,
        mut lymph_nodes: Query<(Entity, &mut LymphNode)>,
    ) -> Result<(), ()> {
        // TODO(pwy) ideally we'd do it only when the node was actually modified
        commands.entity(self.lymph_node).insert(NeedsRecompiling);

        let (_, mut lymph_node) =
            lymph_nodes.get_mut(self.lymph_node).map_err(drop)?;

        let mut open = true;

        egui::Window::new("Lymph Node")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut open)
            .show(egui.ctx_mut(), |ui| {
                egui::Grid::new("lymph-node-editor.recipe").show(ui, |ui| {
                    ui.add(UiLymphNodeInputRadios::new(
                        textures,
                        &mut lymph_node.lhs,
                    ));

                    ui.centered_and_justified(|ui| {
                        ui.label("+");
                    });

                    ui.add(UiLymphNodeInputRadios::new(
                        textures,
                        &mut lymph_node.rhs,
                    ));
                });
            });

        if open {
            Ok(())
        } else {
            Err(())
        }
    }
}
