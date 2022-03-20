use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub trait DebugLinesExt {
    fn square(&mut self, start: Vec2, end: Vec2)
        -> DebugLinesSquareBuilder<'_>;
}

impl DebugLinesExt for DebugLines {
    fn square(
        &mut self,
        start: Vec2,
        end: Vec2,
    ) -> DebugLinesSquareBuilder<'_> {
        DebugLinesSquareBuilder::new(self, start, end)
    }
}

pub struct DebugLinesSquareBuilder<'a> {
    lines: &'a mut DebugLines,
    start: Vec2,
    end: Vec2,
    duration: f32,
    color: Color,
}

impl<'a> DebugLinesSquareBuilder<'a> {
    pub fn new(lines: &'a mut DebugLines, start: Vec2, end: Vec2) -> Self {
        Self {
            lines,
            start,
            end,
            duration: 0.0,
            color: Color::WHITE,
        }
    }
    pub fn duration(mut self, value: f32) -> Self {
        self.duration = value;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn draw(self) {
        let start = self.start.extend(0.0);
        let end = self.end.extend(0.0);
        let right = Vec3::new(end.x - start.x, 0.0, 0.0);
        let up = Vec3::new(0.0, end.y - start.y, 0.0);

        let mut line = |a, b| {
            self.lines.line_colored(a, b, self.duration, self.color);
        };

        line(start, start + right);
        line(start, start + up);
        line(end, end - right);
        line(end, end - up);
    }
}
