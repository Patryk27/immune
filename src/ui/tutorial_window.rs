use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::GameState;

pub fn system(mut state: ResMut<GameState>, mut egui: ResMut<EguiContext>) {
    if !state.tutorial {
        return;
    }

    egui::TopBottomPanel::top("top_panel").show(egui.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("Tutorial");
        });
    });

    egui::TopBottomPanel::bottom("bottop_panel").show(egui.ctx_mut(), |_ui| {});

    egui::CentralPanel::default().show(egui.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
            if ui.button("Skip tutorial").clicked() {
                state.tutorial = false;
            }
        });
    });
}
