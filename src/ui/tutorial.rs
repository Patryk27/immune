use bevy::prelude::*;
use bevy_egui::egui::{TextStyle, Ui, WidgetText};
use bevy_egui::{egui, EguiContext};
use instant::{Duration, Instant};

use crate::game::{GameState, LevelVm};
use crate::tutorial::{TutorialState, TUTORIAL_STAGES};

const LYMPH_TUTORIAL_PIC: u64 = 0;
const ENEMIES_PIC: u64 = 2;
const COMBAT_PIC: u64 = 3;
const UNFAIR_ADVENTAGE_PIC: u64 = 4;

pub fn load_assets(
    mut egui_context: ResMut<EguiContext>,
    assets: Res<AssetServer>,
) {
    let texture_handle = assets.load("tutorial/lymph_tutorial.png");
    egui_context.set_egui_texture(LYMPH_TUTORIAL_PIC, texture_handle);

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

                game_state.vm = LevelVm::AwaitingStart {
                    at: Instant::now() + Duration::from_secs(5),
                };
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
            EnemiesIntroduction => enemies_introduction(ui),
            CombatInstuctions => combat_instuctions(ui),
            EnemiesUnfairAdventage => enemies_unfair_adventage(ui),

            Gameplay => {
                game_state.tutorial = false;

                game_state.vm = LevelVm::AwaitingStart {
                    at: Instant::now() + Duration::from_secs(5),
                };
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
            WidgetText::from("Immune: Unfair Advantage")
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
            [934.0 / 2.0, 715.0 / 2.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Lymph nodes are cells which allows you to build battle unit cells.");
        ui.label("In order to build battle unit cells you have to connect Lymph nodes.");
        ui.label("Each of the Lymph nodes has access to one resource,");
        ui.label("and can forward its oputput to another Lymph node.");
        ui.label("Only by combining different resources, you are able to build battle cells");
        ui.label("Try different combinations!");
    });
}

fn enemies_introduction(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(ENEMIES_PIC),
            [650.0 / 2.0, 557.0 / 2.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Those little cute buddies wants only one thing - to rip you apart.");
        ui.label("Viruses are extremely dangerous and your job is to fight them.");
        ui.label("Dont be fooled by those cute faces.");
    });
}

fn combat_instuctions(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(COMBAT_PIC),
            [1182.0 / 2.0, 955.0 / 2.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("To arms! Create tons of battle cells and protect your host body.");
        ui.label("In order to select your units just drag your mouse with left button pressed.");
        ui.label("You can also select one unit or multiselect with `ctrl` button hold");
        ui.label("To give them order, press right mouse button in the place you want them to be.");
    });
}

fn enemies_unfair_adventage(ui: &mut Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(UNFAIR_ADVENTAGE_PIC),
            [1388.0 / 2.0, 986.0 / 2.0],
        ));
    });

    ui.separator();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label("Enemies are smart and attack in groups but thats not the worse...");
        ui.label("They can reproduce themselves in your Lymph nodes if you react too slow.");
        ui.label("So give them battle from the very beggining otherwise you will rot.");
    });
}
