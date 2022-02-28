use bevy_egui::egui::epaint::{self, Mesh};
use bevy_egui::egui::{
    pos2, vec2, Color32, Pos2, Rect, Response, Sense, Shape, TextureId, Ui,
    Vec2, Widget,
};

pub struct UiRadioImageButton {
    checked: bool,
    texture_id: TextureId,
    image_size: Vec2,
    image_padding: Vec2,
    image_tint: Color32,
}

impl UiRadioImageButton {
    pub fn new(
        checked: bool,
        texture_id: TextureId,
        image_size: impl Into<Vec2>,
    ) -> Self {
        Self {
            checked,
            texture_id,
            image_size: image_size.into(),
            image_padding: Default::default(),
            image_tint: Color32::WHITE,
        }
    }

    pub fn with_image_padding(mut self, val: Vec2) -> Self {
        self.image_padding = val;
        self
    }

    pub fn with_image_tint(mut self, val: Color32) -> Self {
        self.image_tint = val;
        self
    }
}

impl Widget for UiRadioImageButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            checked,
            texture_id,
            image_size,
            image_padding,
            image_tint,
        } = self;

        let radio_size = {
            let icon_width = ui.spacing().icon_width;
            let icon_spacing = ui.spacing().icon_spacing;
            let button_padding = ui.spacing().button_padding;

            button_padding
                + vec2(icon_width + icon_spacing, 0.0)
                + button_padding
        };

        let (rect, response) = ui.allocate_exact_size(
            radio_size + image_size + image_padding,
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let visuals = ui.style().interact(&response);

            // ----- //
            // Radio //

            let (inner_rect, outer_rect) = ui.spacing().icon_rectangles(rect);

            painter.add(epaint::CircleShape {
                center: outer_rect.center(),
                radius: outer_rect.width() / 2.0 + visuals.expansion,
                fill: visuals.bg_fill,
                stroke: visuals.bg_stroke,
            });

            if checked {
                painter.add(epaint::CircleShape {
                    center: inner_rect.center(),
                    radius: inner_rect.width() / 3.0,
                    fill: visuals.fg_stroke.color,
                    stroke: Default::default(),
                });
            }

            // ----- //
            // Image //

            let image_rect = Rect {
                min: Pos2 {
                    x: rect.min.x + radio_size.x + (image_padding.x / 2.0),
                    y: rect.min.y + (image_padding.y / 2.0),
                },
                max: rect.max - (image_padding / 2.0),
            };

            let image_uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));

            painter.add(Shape::mesh({
                let mut mesh = Mesh::with_texture(texture_id);
                mesh.add_rect_with_uv(image_rect, image_uv, image_tint);
                mesh
            }));
        }

        response
    }
}
