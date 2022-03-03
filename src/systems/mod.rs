use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub mod bio;
pub mod camera;
pub mod debug;
pub mod highlight;
pub mod input;
pub mod physics;
pub mod units;

pub fn draw_square(lines: &mut DebugLines, start_point: Vec2, end_point: Vec2) {
    let start_point = start_point.extend(0.0);
    let end_point = end_point.extend(0.0);
    let right = Vec3::new(end_point.x - start_point.x, 0.0, 0.0);
    let up = Vec3::new(0.0, end_point.y - start_point.y, 0.0);

    lines.line(start_point, start_point + right, 0.0);
    lines.line(start_point, start_point + up, 0.0);
    lines.line(end_point, end_point - right, 0.0);
    lines.line(end_point, end_point - up, 0.0);
}
