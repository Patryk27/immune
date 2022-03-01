use bevy::prelude::*;
use bevy_egui::egui::Color32;

pub fn to_egui(color: Color) -> Color32 {
    let [r, g, b, _] = color.as_rgba_f32();
    Color32::from_rgb((255.0 * r) as _, (255.0 * g) as _, (255.0 * b) as _)
}

pub mod ui {
    use super::*;

    pub fn text_danger() -> Color {
        Color::rgb_u8(220, 53, 69)
    }

    pub fn text_danger_egui() -> Color32 {
        to_egui(text_danger())
    }
}

pub mod z_index {
    pub const CELL: f32 = 1.0;
    pub const LYMPH_NODE: f32 = 0.9;
    pub const LYMPH_NODE_COMPILATION_WARNING: f32 = 1.1;
}
