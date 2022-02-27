use crate::systems::cell_node::{
    Leukocyte, LeukocyteKind, LymphNodeInput, LymphNodeOutput,
};

pub struct Compiler {
    //
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(
        &self,
        lhs: Option<LymphNodeInput>,
        rhs: Option<LymphNodeInput>,
    ) -> Option<LymphNodeOutput> {
        use LymphNodeInput as LNI;

        // TODO(pwy) figure out if compiling `None`-s could make sense
        let (lhs, rhs) = (lhs?, rhs?);

        let body = match (lhs, rhs) {
            (LNI::Body(body), _) | (_, LNI::Body(body)) => Some(body),
            (LNI::External(_), _) | (_, LNI::External(_)) => todo!(),
            _ => None,
        };

        let binder = match (lhs, rhs) {
            (LNI::Binder(binder), _) | (_, LNI::Binder(binder)) => Some(binder),
            (LNI::External(_), _) | (_, LNI::External(_)) => todo!(),
            _ => None,
        };

        let (body, binder) = (body?, binder?);

        Some(LymphNodeOutput::Leukocyte(Leukocyte {
            body,
            binder,
            kind: LeukocyteKind::Killer,
        }))
    }
}
