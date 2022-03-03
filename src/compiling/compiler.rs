//! TODO(pwy) memoization and some other optimizations could be nice

use std::collections::{BTreeMap, HashSet};

use bevy::prelude::Entity;
use itertools::Itertools;

use crate::systems::bio::{
    Leukocyte, LeukocyteKind, LeukocyteProps, LymphNode, LymphNodeFunction,
    LymphNodeInput, LymphNodeOutput, LymphNodeState, Protein,
};

#[derive(Default)]
pub struct Compiler {
    nodes: BTreeMap<Entity, CompilerLymphNode>,
}

struct CompilerLymphNode {
    lhs: Option<LymphNodeInput>,
    rhs: Option<LymphNodeInput>,
    function: LymphNodeFunction,
    state: LymphNodeState,
}

const MAX_DEPTH: u8 = 128;

impl Compiler {
    pub fn add(&mut self, entity: Entity, node: &LymphNode) {
        self.nodes.insert(
            entity,
            CompilerLymphNode {
                lhs: node.lhs,
                rhs: node.rhs,
                function: LymphNodeFunction::Producer,
                state: node.state,
            },
        );
    }

    pub fn compile(
        mut self,
    ) -> Vec<(
        Entity,
        Option<LymphNodeOutput>,
        LymphNodeState,
        LymphNodeFunction,
    )> {
        self.resolve_functions();

        self.nodes
            .keys()
            .cloned()
            .collect_vec()
            .into_iter()
            .map(|entity| {
                (
                    entity,
                    self.resolve_output(0, entity),
                    self.resolve_state(entity),
                    self.nodes[&entity].function,
                )
            })
            .collect()
    }

    fn resolve_functions(&mut self) {
        let providers: HashSet<_> = self
            .nodes
            .iter()
            .flat_map(|(_, node)| {
                let lhs = node.lhs.into_iter();
                let rhs = node.rhs.into_iter();

                lhs.chain(rhs)
            })
            .flat_map(|input| {
                if let LymphNodeInput::External(Some(entity)) = input {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect();

        for provider in providers {
            self.nodes.get_mut(&provider).unwrap().function =
                LymphNodeFunction::Supplier;
        }
    }

    fn resolve_output(
        &self,
        depth: u8,
        entity: Entity,
    ) -> Option<LymphNodeOutput> {
        use LymphNodeInput as I;

        if depth > MAX_DEPTH {
            return None;
        }

        let entity = &self.nodes[&entity];
        let lhs = entity.lhs;
        let rhs = entity.rhs;

        match (lhs, rhs) {
            // (Body + Binder) | (Binder + Body) => Leukocyte
            (Some(I::Body(body)), Some(I::Binder(binder)))
            | (Some(I::Binder(binder)), Some(I::Body(body))) => {
                Some(LymphNodeOutput::Leukocyte(Leukocyte {
                    body,
                    binder,
                    kind: LeukocyteKind::Killer,
                    props: LeukocyteProps { hp: 10 },
                }))
            }

            // (Leukocyte + Protein) | (Protein + Leukocyte) => Leukocyte + Extra
            (Some(I::External(Some(source))), Some(I::Protein(protein)))
            | (Some(I::Protein(protein)), Some(I::External(Some(source)))) => {
                let LymphNodeOutput::Leukocyte(mut leukocyte) =
                    self.resolve_output(depth + 1, source)?;

                // TODO(pwy) they should do something unique
                match protein {
                    Protein::Dumbbell => {
                        leukocyte.props.hp += 5;
                    }
                    Protein::Star => {
                        leukocyte.props.hp += 10;
                    }
                }

                Some(LymphNodeOutput::Leukocyte(leukocyte))
            }

            // (Leukocyte + Leukocyte) | (Leukocyte + Leukocyte) => Extra
            (Some(I::External(Some(lhs))), Some(I::External(Some(rhs)))) => {
                let LymphNodeOutput::Leukocyte(lhs) =
                    self.resolve_output(depth + 1, lhs)?;

                let LymphNodeOutput::Leukocyte(rhs) =
                    self.resolve_output(depth + 1, rhs)?;

                if lhs.body == rhs.body
                    && lhs.binder == rhs.binder
                    && lhs.kind == rhs.kind
                {
                    Some(LymphNodeOutput::Leukocyte(Leukocyte {
                        body: lhs.body,
                        binder: lhs.binder,
                        kind: lhs.kind,
                        props: LeukocyteProps {
                            hp: ((lhs.props.hp as f32 + rhs.props.hp as f32)
                                * 1.5) as _,
                        },
                    }))
                } else {
                    None
                }
            }

            // (None + External) | (External + None) => Pass-through
            (Some(I::External(Some(source))), None)
            | (None, Some(I::External(Some(source)))) => {
                self.resolve_output(depth + 1, source)
            }

            _ => None,
        }
    }

    fn resolve_state(&self, entity: Entity) -> LymphNodeState {
        fn is_paused(
            nodes: &BTreeMap<Entity, CompilerLymphNode>,
            depth: u8,
            entity: Entity,
        ) -> bool {
            if depth > MAX_DEPTH {
                return false;
            }

            let node = &nodes[&entity];

            if node.state.paused && depth > 0 {
                return true;
            }

            if let Some(LymphNodeInput::External(Some(lhs))) = node.lhs {
                if is_paused(nodes, depth + 1, lhs) {
                    return true;
                }
            }

            if let Some(LymphNodeInput::External(Some(rhs))) = node.rhs {
                if is_paused(nodes, depth + 1, rhs) {
                    return true;
                }
            }

            false
        }

        let mut state = self.nodes[&entity].state;

        state.awaiting_resources = is_paused(&self.nodes, 0, entity);
        state
    }
}
