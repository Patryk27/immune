use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::systems::bio::{LymphNode, LymphNodeTarget};
use crate::ui::Poll;

pub struct UiLymphNodePicker {
    alive: bool,
    target_node_entity: Option<Entity>,
}

impl UiLymphNodePicker {
    pub fn new() -> Self {
        Self {
            alive: true,
            target_node_entity: None,
        }
    }

    pub fn process(
        &mut self,
        mut lines: ResMut<DebugLines>,
        mouse_pos: Vec2,
        lymph_nodes: &mut Query<(
            &mut LymphNode,
            &Transform,
            &Children,
            Entity,
        )>,
        source_node_entity: Entity,
    ) -> Poll<Option<Entity>> {
        if let Some(target_node_entity) = self.target_node_entity {
            if target_node_entity == source_node_entity {
                self.target_node_entity = None;
            }
        }

        if !self.alive {
            return Poll::Ready(self.target_node_entity);
        }

        if let Some(target_node_entity) = self.target_node_entity {
            let result: Result<_, QueryEntityError> = try {
                let (target_node, _, _, _) =
                    lymph_nodes.get_mut(target_node_entity)?;

                if let Some(parent_node_entity) = target_node.parent {
                    let (mut parent_node, _, _, _) =
                        lymph_nodes.get_mut(parent_node_entity)?;

                    parent_node.target = LymphNodeTarget::Outside;
                }

                let (mut source_node, _, _, _) =
                    lymph_nodes.get_mut(source_node_entity)?;

                source_node.target =
                    LymphNodeTarget::LymphNode(target_node_entity);
            };

            if result.is_err() {
                // Some of the lymph nodes don't exist no more - what a pity!
                Poll::Ready(None)
            } else {
                Poll::Ready(self.target_node_entity)
            }
        } else {
            let (_, source_node_transform, _, _) =
                lymph_nodes.get_mut(source_node_entity).unwrap();

            lines.line(
                source_node_transform.translation.truncate().extend(5.0),
                mouse_pos.extend(5.0),
                0.0,
            );

            Poll::Pending
        }
    }

    pub fn on_escape_pressed(&mut self) {
        self.alive = false;
    }

    pub fn on_lymph_node_clicked(&mut self, node: Entity) {
        self.target_node_entity = Some(node);
    }
}
