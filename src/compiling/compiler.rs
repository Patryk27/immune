use std::collections::BTreeMap;

use bevy::prelude::Entity;
use itertools::Itertools;

use crate::systems::cell_node::{
    Leukocyte, LeukocyteKind, LeukocyteProps, LymphNode, LymphNodeInput,
    LymphNodeOutput, Protein,
};

#[derive(Default)]
pub struct Compiler {
    nodes: BTreeMap<Entity, CompilerLymphNode>,
}

struct CompilerLymphNode {
    lhs: Option<LymphNodeInput>,
    rhs: Option<LymphNodeInput>,
    output: CompilerLymphNodeOutput,
}

enum CompilerLymphNodeOutput {
    Pending,
    Compiled(Option<LymphNodeOutput>),
}

impl Compiler {
    pub fn add(&mut self, entity: Entity, node: &LymphNode) {
        self.nodes.insert(
            entity,
            CompilerLymphNode {
                lhs: node.lhs,
                rhs: node.rhs,
                output: CompilerLymphNodeOutput::Pending,
            },
        );
    }

    pub fn compile(mut self) -> Vec<(Entity, Option<LymphNodeOutput>)> {
        self.nodes
            .keys()
            .cloned()
            .collect_vec()
            .into_iter()
            .map(|entity| (entity, self.resolve(0, entity)))
            .collect()
    }

    fn resolve(
        &mut self,
        depth: u8,
        entity: Entity,
    ) -> Option<LymphNodeOutput> {
        assert!(depth < 128);

        let node = &self.nodes[&entity];

        match node.output {
            CompilerLymphNodeOutput::Pending => {
                let (lhs, rhs) = (node.lhs, node.rhs);
                let output = self.resolve_inner(depth, lhs, rhs);

                self.nodes.get_mut(&entity).unwrap().output =
                    CompilerLymphNodeOutput::Compiled(output);

                output
            }

            CompilerLymphNodeOutput::Compiled(output) => output,
        }
    }

    fn resolve_inner(
        &mut self,
        depth: u8,
        lhs: Option<LymphNodeInput>,
        rhs: Option<LymphNodeInput>,
    ) -> Option<LymphNodeOutput> {
        use LymphNodeInput as I;

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

            // (External + Protein) | (Protein + External) => Extra
            (Some(I::External(source)), Some(I::Protein(protein)))
            | (Some(I::Protein(protein)), Some(I::External(source))) => {
                let LymphNodeOutput::Leukocyte(mut leukocyte) =
                    self.resolve(depth + 1, source)?;

                // TODO(pwy) they should do something unique
                match protein {
                    Protein::Dumbbell => {
                        leukocyte.props.hp += 2;
                    }
                    Protein::Star => {
                        leukocyte.props.hp += 4;
                    }
                }

                Some(LymphNodeOutput::Leukocyte(leukocyte))
            }

            // (External + External) | (External + External) => Extra
            (Some(I::External(lhs)), Some(I::External(rhs))) => {
                let LymphNodeOutput::Leukocyte(lhs) =
                    self.resolve(depth + 1, lhs)?;

                let LymphNodeOutput::Leukocyte(rhs) =
                    self.resolve(depth + 1, rhs)?;

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
                                * 1.25) as _,
                        },
                    }))
                } else {
                    None
                }
            }

            // (None + External) | (External + None) => Pass-through
            (Some(I::External(source)), None)
            | (None, Some(I::External(source))) => {
                self.resolve(depth + 1, source)
            }

            _ => None,
        }
    }
}
