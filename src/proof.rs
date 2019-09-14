use crate::qbf::{Clause, Literal, CNF, QBF};
use std::collections::{HashMap, HashSet};

trait Proof {
    fn caused_changes(&self, formula: &QBF) -> Option<Vec<Change>>;
}

enum Change {
    ClauseAddition(Clause),          // Clause which is to be added
    ClauseRemoval(Clause),           // Clause which is to be removed
    LiteralRemoval(Clause, Literal), // Clause out of Literal is to be removed
}

impl Change {
    fn apply(&self, formula: &mut QBF) -> bool {
        match self {
            Change::ClauseAddition(c) => formula.add_clause(c),
            Change::ClauseRemoval(c) => formula.remove_clause(c),
            Change::LiteralRemoval(c, l) => formula.remove_literal(c.clone(), l),
        }
    }
}

struct AllExpResProof {
    rules: Vec<AllExpResRule>,
}

enum AllExpResRule {
    Axiom(Clause),                                // Clause used for the axiom rule
    Resolution(Clause, Literal, Clause, Literal), // (Clause, Literal) pairs used for resolution
}

fn axiom(clause: Clause, formula: &QBF) -> Option<Vec<Change>> {
    let CNF(clauses) = &formula.cnf;
    if !clauses.contains(&clause) {
        return None;
    }
    let Clause(literals) = &clause;
    let mut new_literals = HashSet::new();
    for l in literals.iter() {
        if l.is_existential(&formula.vars) {
            new_literals.insert(l.clone()); // TODO add assignment
        }
    }
    Some(vec![Change::ClauseAddition(Clause(new_literals))])
}
