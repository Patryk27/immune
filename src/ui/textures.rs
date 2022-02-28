use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::egui::TextureId;
use bevy_egui::EguiContext;

#[derive(Default)]
pub struct UiTextures {
    textures: HashMap<String, TextureId>,
}

impl UiTextures {
    pub fn load(
        &mut self,
        assets: &AssetServer,
        egui: &mut EguiContext,
        path: impl AsRef<str>,
    ) {
        let path = path.as_ref();
        let next_id = (self.textures.len() + 1) as u64;

        self.textures.entry(path.to_owned()).or_insert_with(|| {
            egui.set_egui_texture(next_id, assets.load(path));
            TextureId::User(next_id)
        });
    }

    pub fn get(&self, path: impl AsRef<str>) -> TextureId {
        let path = path.as_ref();

        *self
            .textures
            .get(path)
            .unwrap_or_else(|| panic!("UiTexture not loaded: {}", path))
    }
}
