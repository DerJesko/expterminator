use crate::qbf::{Clause, CNF, QBF};

enum Change {
    ClauseAddition(Clause),       // Clause which is to be added
    ClauseRemoval(usize),         // Index of clause which is to be removed
    LiteralRemoval(usize, usize), // (Clause, Literal) index of literal which is to be removed
}

impl Change {
    fn apply(&self, formula: &mut QBF) -> bool {
        match self {
            Change::ClauseAddition(c) => formula.add_clause(c),
            Change::ClauseRemoval(i) => formula.remove_clause(*i),
            Change::LiteralRemoval(c_index, l_index) => formula.remove_literal(*c_index, *l_index),
        }
    }
}

struct AllExpResProof {
    rules: Vec<AllExpResRule>,
}

enum AllExpResRule {
    Axiom(usize),                           // Index of the clause used for the axiom rule
    Resolution(usize, usize, usize, usize), // Clause and literal index of the two literals used for resolution
}
