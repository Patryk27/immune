use bevy::prelude::*;
use bevy_egui::egui::{TextStyle, Ui, WidgetText};
use bevy_egui::{egui, EguiContext};

use crate::game::GameState;
use crate::tutorial::{TutorialState, TUTORIAL_STAGES};

const LYMPH_TUTORIAL_PIC: u64 = 0;
const RESOURCE_TUTORIAL_PIC: u64 = 1;
const ENEMIES_PIC: u64 = 2;
const COMBAT_PIC: u64 = 3;
const UNFAIR_ADVENTAGE_PIC: u64 = 4;

pub fn load_assets(
    mut egui_context: ResMut<EguiContext>,
    assets: Res<AssetServer>,
) {
    let texture_handle = assets.load("tutorial/lymph_tutorial.png");
    egui_context.set_egui_texture(LYMPH_TUTORIAL_PIC, texture_handle);

    let texture_handle = assets.load("biohazard-symbol.png");
    egui_context.set_egui_texture(RESOURCE_TUTORIAL_PIC, texture_handle);

    let texture_handle = assets.load("tutorial/enemies.png");
    egui_context.set_egui_texture(ENEMIES_PIC, texture_handle);

    let texture_handle = assets.load("tutorial/combat_tutorial.png");
    egui_context.set_egui_texture(COMBAT_PIC, texture_handle);

    let texture_handle = assets.load("tutorial/viruses_attack_ua.png");
    egui_context.set_egui_texture(UNFAIR_ADVENTAGE_PIC, texture_handle);
}

pub fn system(
    mut game_state: ResMut<GameState>,
    mut tutorial_state: ResMut<TutorialState>,
    mut egui: ResMut<EguiContext>,
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
            ui.label(format!(
                "{}/{}",
                tutorial_state.stage + 1,
                TUTORIAL_STAGES.len() - 1
            ));
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

        match stage {
            WeclomePage => welcome_page(ui),
            LymphNodeIntroduction => lymph_node_introduction(ui),
            ResourcesIntroduction => resources_introduction(ui),
            EnemiesIntroduction => enemies_introduction(ui),
            CombatInstuctions => combat_instuctions(ui),
            EnemiesUnfairAdventage => enemies_unfair_adventage(ui),
            Gameplay => {
                game_state.tutorial = false;
            }
        }
    });
}

fn welcome_page(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label(
            WidgetText::from("Immune: Unfair Adventage")
                .text_style(TextStyle::Heading),
        );
        ui.label("");
        ui.label("");
        ui.label("by");
        ui.label("Jakub Trąd");
        ui.label("Patryk Wychowaniec");
        ui.label("Paweł Rynowiecki");
    });
}

fn lymph_node_introduction(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(LYMPH_TUTORIAL_PIC),
            [934.0, 715.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor ");
        ui.label("incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis ");
        ui.label("nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
        ui.label("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu ");
        ui.label("fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa");
        ui.label("qui officia deserunt mollit anim id est laborum.");
    });
}

fn resources_introduction(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(RESOURCE_TUTORIAL_PIC),
            [715.0, 715.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor ");
        ui.label("incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis ");
        ui.label("nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
        ui.label("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu ");
        ui.label("fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa");
        ui.label("qui officia deserunt mollit anim id est laborum.");
    });
}

fn enemies_introduction(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(ENEMIES_PIC),
            [650.0, 557.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor ");
        ui.label("incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis ");
        ui.label("nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
        ui.label("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu ");
        ui.label("fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa");
        ui.label("qui officia deserunt mollit anim id est laborum.");
    });
}

fn combat_instuctions(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(COMBAT_PIC),
            [1182.0, 955.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor ");
        ui.label("incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis ");
        ui.label("nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
        ui.label("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu ");
        ui.label("fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa");
        ui.label("qui officia deserunt mollit anim id est laborum.");
    });
}

fn enemies_unfair_adventage(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(UNFAIR_ADVENTAGE_PIC),
            [1388.0, 986.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor ");
        ui.label("incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis ");
        ui.label("nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
        ui.label("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu ");
        ui.label("fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa");
        ui.label("qui officia deserunt mollit anim id est laborum.");
    });
}
