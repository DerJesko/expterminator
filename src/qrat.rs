use crate::qbf::Clause;

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
