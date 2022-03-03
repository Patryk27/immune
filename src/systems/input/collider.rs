use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum Collider {
    Circle { radius: f32 },
}

impl Collider {
    pub fn contains(self, object: Vec2, mouse: Vec2) -> bool {
        match self {
            Self::Circle { radius } => point_in_circle(mouse, object, radius),
        }
    }
}

fn point_in_circle(
    point: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
) -> bool {
    point.distance(circle_center) <= circle_radius
}
