use std::collections::BTreeMap;

use bevy::prelude::Entity;
use itertools::Itertools;

use crate::systems::bio::{
    Leukocyte, LeukocyteKind, LeukocyteProps, LymphNode, LymphNodeProduct,
    LymphNodeResource, LymphNodeState, LymphNodeTarget, Protein,
};

#[derive(Default)]
pub struct Compiler {
    nodes: BTreeMap<Entity, CachedLymphNode>,
    parents: BTreeMap<Entity, Entity>,
}

struct CachedLymphNode {
    resource: Option<LymphNodeResource>,
    state: LymphNodeState,
}

const MAX_DEPTH: u8 = 128;

impl Compiler {
    pub fn add(&mut self, entity: Entity, node: &LymphNode) {
        self.nodes.insert(
            entity,
            CachedLymphNode {
                resource: node.resource,
                state: LymphNodeState {
                    is_paused: node.state.is_paused,
                    is_awaiting_resources: false,
                },
            },
        );

        if let LymphNodeTarget::LymphNode(child) = node.target {
            self.parents.insert(child, entity);
        }
    }

    pub fn compile(
        self,
    ) -> BTreeMap<
        Entity,
        (Option<LymphNodeProduct>, Option<Entity>, LymphNodeState),
    > {
        self.nodes
            .keys()
            .cloned()
            .collect_vec()
            .into_iter()
            .map(|entity| {
                let product = self.resolve_product(0, entity);
                let parent = self.parents.get(&entity).cloned();
                let state = self.resolve_state(entity);

                (entity, (product, parent, state))
            })
            .collect()
    }

    // TODO(pwy) add more combinations
    // TODO(pwy) memoization would be nice
    fn resolve_product(
        &self,
        depth: u8,
        entity: Entity,
    ) -> Option<LymphNodeProduct> {
        use {LymphNodeProduct as P, LymphNodeResource as R};

        if depth > MAX_DEPTH {
            return None;
        }

        let node = &self.nodes[&entity];

        let lhs = self
            .parents
            .get(&entity)
            .and_then(|&parent| self.resolve_product(depth + 1, parent));

        let rhs = node.resource;

        let (lhs, rhs) = match (lhs, rhs) {
            (None, None) => {
                return None;
            }
            (None, Some(rhs)) => {
                return Some(LymphNodeProduct::Resource(rhs));
            }
            (Some(lhs), None) => {
                return Some(lhs);
            }
            (Some(lhs), Some(rhs)) => (lhs, rhs),
        };

        match (lhs, rhs) {
            (P::Resource(res1), res2) => {
                if let (R::Antigen(binder), R::Body(body))
                | (R::Body(body), R::Antigen(binder)) = (res1, res2)
                {
                    Some(P::Leukocyte(Leukocyte {
                        body,
                        binder,
                        kind: LeukocyteKind::Killer,
                        props: LeukocyteProps { hp: 10 },
                    }))
                } else {
                    None
                }
            }

            (P::Leukocyte(mut cell), res) => match res {
                R::Body(_) => {
                    cell.props.hp += 5;
                    Some(P::Leukocyte(cell))
                }

                R::Antigen(_) => {
                    cell.props.hp += 2;
                    Some(P::Leukocyte(cell))
                }

                R::Protein(prot) => match prot {
                    Protein::Dumbbell => {
                        cell.props.hp += 10;
                        Some(P::Leukocyte(cell))
                    }

                    Protein::Star => {
                        cell.props.hp *= 2;
                        Some(P::Leukocyte(cell))
                    }
                },
            },
        }
    }

    fn resolve_state(&self, entity: Entity) -> LymphNodeState {
        let mut state = self.nodes[&entity].state;

        {
            let mut depth = 0;
            let mut node = entity;

            loop {
                if depth > MAX_DEPTH {
                    break;
                }

                if let Some(parent) = self.parents.get(&node) {
                    node = *parent;
                } else {
                    break;
                }

                if depth > 0 && self.nodes[&node].state.is_paused {
                    state.is_awaiting_resources = true;
                    break;
                }

                depth += 1;
            }
        }

        state
    }
}
