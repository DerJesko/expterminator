use crate::qbf::{Clause, Literal, CNF, QBF};
use std::collections::{HashMap, HashSet};

/*
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
*/

enum QRATRule {
    UnitPropagation(Literal),
}

impl QRATRule {
    pub fn apply(&self, formula: QBF) -> Option<QBF> {
        match self {
            QRATRule::UnitPropagation(literal) => QRATRule::unit_propagate(literal, formula), // TODO rest of the rules
        }
    }

    fn unit_propagate(literal: &Literal, formula: QBF) -> Option<QBF> {
        let CNF(mut clauses) = formula.cnf;
        let mut unit_clause = HashSet::new();
        unit_clause.insert(literal.clone());
        if !clauses.contains(&Clause(unit_clause)) {
            return None;
        }
        let mut new_clauses = HashSet::new();
        for clause in clauses.drain() {
            let Clause(mut literals) = clause;
            literals.retain(|literal| !literal.is_inverse(literal));
            new_clauses.insert(Clause(literals));
        }
        return Some(QBF {
            vars: formula.vars,
            cnf: CNF(new_clauses),
        });
    }
}

struct AllExpResProof {
    rules: Vec<AllExpResRule>,
}

impl AllExpResProof {
    pub fn apply(&self, mut formula: QBF) -> Option<QBF> {
        for r in &self.rules {
            match r.apply(formula) {
                Some(f) => formula = f,
                None => return None,
            }
        }
        Some(formula)
    }

    pub fn check(&self, mut formula: QBF) -> bool {
        false // TODO
    }
}

enum AllExpResRule {
    Axiom(Clause),                                // Clause used for the axiom rule
    Resolution(Clause, Literal, Clause, Literal), // (Clause, Literal) pairs used for resolution
}

impl AllExpResRule {
    pub fn apply(&self, formula: QBF) -> Option<QBF> {
        match self {
            AllExpResRule::Axiom(clause) => AllExpResRule::axiom(clause.clone(), formula),
            AllExpResRule::Resolution(c1, l1, c2, l2) => {
                AllExpResRule::resolution(c1.clone(), l1, c2.clone(), l2, formula)
            }
        }
    }

    fn axiom(clause: Clause, mut formula: QBF) -> Option<QBF> {
        let CNF(clauses) = &formula.cnf;
        if !clauses.contains(&clause) {
            return None;
        }
        let Clause(literals) = &clause;
        let mut new_literals = HashSet::new();
        for l1 in literals.iter() {
            if l1.is_existential(&formula.vars) {
                let mut map = HashMap::new();
                for l2 in literals.iter() {
                    if !l2.is_existential(&formula.vars) && l2.less(l1, &formula.vars) {
                        map.insert(l2.variable, !l2.positive);
                    }
                }
                new_literals.insert(l1.clone()); // TODO add assignment
            }
        }
        if !formula.add_clause(&Clause(new_literals)) {
            return None;
        }
        Some(formula)
    }

    fn resolution(
        mut c1: Clause,
        l1: &Literal,
        mut c2: Clause,
        l2: &Literal,
        mut formula: QBF,
    ) -> Option<QBF> {
        let CNF(clauses) = &formula.cnf;
        {
            let Clause(literals1) = &mut c1;
            let Clause(literals2) = &mut c2;
            if !literals1.remove(&l1) || !literals2.remove(&l2) {
                return None;
            }
        }
        if !clauses.contains(&c1) || !clauses.contains(&c2) || !l1.is_inverse(&l2) {
            return None;
        }

        let Clause(literals1) = c1;
        let Clause(literals2) = &c2;
        literals1.union(&literals2);
        if !formula.add_clause(&Clause(literals1)) {
            return None;
        }
        Some(formula)
    }
}
