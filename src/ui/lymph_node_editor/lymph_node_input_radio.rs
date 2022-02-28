use std::iter;

use bevy_egui::egui::{vec2, Color32, RadioButton, Response, Ui, Vec2, Widget};

use crate::systems::cell_node::LymphNodeInput;
use crate::ui::{UiRadioImageButton, UiTextures};

#[derive(Clone, Copy)]
pub struct UiLymphNodeInputRadio<'a> {
    textures: &'a UiTextures,
    checked: bool,
    value: Option<LymphNodeInput>,
}

impl<'a> UiLymphNodeInputRadio<'a> {
    pub fn variants(
        textures: &'a UiTextures,
        selected_value: Option<LymphNodeInput>,
    ) -> impl Iterator<Item = UiLymphNodeInputRadio<'a>> {
        iter::once(None)
            .chain(LymphNodeInput::variants().map(Some))
            .map(move |value| Self {
                textures,
                checked: value == selected_value,
                value,
            })
    }

    pub fn value(&self) -> Option<LymphNodeInput> {
        self.value
    }

    fn asset_path(&self) -> Option<&'static str> {
        self.value.map(|value| value.asset_path())
    }

    fn size(&self) -> Vec2 {
        match self.value {
            Some(LymphNodeInput::Binder(_)) => vec2(30.0, 30.0),
            Some(LymphNodeInput::Protein(_)) => vec2(18.0, 18.0),
            _ => vec2(50.0, 50.0),
        }
    }

    fn tint(&self) -> Color32 {
        match self.value {
            Some(LymphNodeInput::Binder(_)) => Color32::GREEN,
            Some(LymphNodeInput::Protein(_)) => Color32::GOLD,
            _ => Color32::WHITE,
        }
    }
}

impl Widget for UiLymphNodeInputRadio<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            if let Some(asset_path) = self.asset_path() {
                let size = self.size();
                let padding = vec2(50.0 - size.x, 5.0);

                ui.add(
                    UiRadioImageButton::new(
                        self.checked,
                        self.textures.get(asset_path),
                        size,
                    )
                    .with_image_padding(padding)
                    .with_image_tint(self.tint()),
                )
            } else {
                ui.add(RadioButton::new(self.checked, "  (nothing)"))
            }
        })
        .inner
    }
}
