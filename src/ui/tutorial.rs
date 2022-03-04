use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{game::GameState, tutorial::{TutorialState, TUTORIAL_STAGES}};

pub fn system(
    mut game_state: ResMut<GameState>,
    mut tutorial_state: ResMut<TutorialState>,
    mut egui: ResMut<EguiContext>
) {
    if !game_state.tutorial {
        return;
    }


    egui::TopBottomPanel::top("top_panel").show(egui.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("Tutorial");
        });
    });


    egui::TopBottomPanel::bottom("bottom_panel").show(egui.ctx_mut(), |ui| {

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("Tutorial progress placeholder");
        });
    });


    egui::TopBottomPanel::bottom("buttons_panel").show(egui.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            if ui.button("Next").clicked() {
                tutorial_state.stage += 1;
            }
            if ui.button("Skip tutorial").clicked() {
                game_state.tutorial = false;
            }
            if tutorial_state.stage > 0 {
                if ui.button("Back").clicked() {
                    tutorial_state.stage -= 1;
                }
            }
        });
    });

    egui::CentralPanel::default().show(egui.ctx_mut(), |ui| {
        use crate::tutorial::TutorialStage::*;
        let stage = TUTORIAL_STAGES[tutorial_state.stage];

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(stage.description());
        });

        ui.separator();


        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("Tutorial step placeholder");
        });

        match stage {
            WeclomePage => (),
            LymphNodeIntroduction => (),
            LymhNodeConnecting => (),
            LyphNodeUnitsProduction => (),
            ResourcesIntroduction => (),
            UnitsControls => (),
            EnemiesIntroduction => (),
            CombatInstuctions => (),
            EnemiesUnfairAdventageExplanation => (),
            Gameplay => {
                game_state.tutorial = false;
            }
        }
    });
}
