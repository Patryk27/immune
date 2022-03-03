use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::systems::bio::{LymphNode, LymphNodeInput};

pub struct UiLymphNodePicker {
    alive: bool,
    input: Input,
    node: Option<Entity>,
}

pub enum UiLymphNodePickerOutcome {
    Awaiting,
    Completed,
}

enum Input {
    Lhs,
    Rhs,
}

impl UiLymphNodePicker {
    pub fn lhs() -> Self {
        Self {
            alive: true,
            input: Input::Lhs,
            node: None,
        }
    }

    pub fn rhs() -> Self {
        Self {
            alive: true,
            input: Input::Rhs,
            node: None,
        }
    }

    pub fn process(
        &mut self,
        mut lines: ResMut<DebugLines>,
        mouse_pos: Vec2,
        lymph_node: &mut LymphNode,
        lymph_node_entity: Entity,
        lymph_node_transform: Transform,
    ) -> UiLymphNodePickerOutcome {
        if !self.alive {
            return UiLymphNodePickerOutcome::Completed;
        }

        if let Some(node) = self.node {
            if node == lymph_node_entity {
                self.node = None;
            }
        }

        if let Some(node) = self.node {
            let input = match self.input {
                Input::Lhs => &mut lymph_node.lhs,
                Input::Rhs => &mut lymph_node.rhs,
            };

            *input = Some(LymphNodeInput::External(Some(node)));

            UiLymphNodePickerOutcome::Completed
        } else {
            lines.line(
                lymph_node_transform.translation.truncate().extend(5.0),
                mouse_pos.extend(5.0),
                0.0,
            );

            UiLymphNodePickerOutcome::Awaiting
        }
    }

    pub fn on_escape_pressed(&mut self) {
        self.alive = false;
    }

    pub fn on_lymph_node_clicked(&mut self, node: Entity) {
        self.node = Some(node);
    }
}
