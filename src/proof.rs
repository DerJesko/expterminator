use crate::qbf::{Clause, QBF};

trait Proof {
    fn apply(&self, formula: &mut QBF);
}

// TODO the proofs also need the witnesses for the steps

struct QRATProof {
    rules: Vec<QRATRule>,
}

enum QRATRule {
    UnitPropagation(Clause), // The clause which is to be added
    QRATAddition(Clause),    // The clause which is to be added
    ClauseRemoval(usize),    // The index of the clause which is to be removed
    QRATLiteralRemoval(
        usize, // The index of the clause from which a literal is to be removed
        usize, // The index of the literal which is to be removed inside the clause
    ),
    UniversalLiteralRemoval(
        usize, // The index of the clause from which a literal is to be removed
        usize, // The index of the literal which is to be removed inside the clause
    ),
}

struct AllExpResProof {
    rules: Vec<AllExpResRule>,
}

enum AllExpResRule {
    Axiom(Clause),      // The clause which is created by the axiom rule
    Resolution(Clause), // The clause which is created by resolution
}
