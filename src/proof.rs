use crate::literal::Literal;
use crate::qbf::{Clause, CNF, QBF};
use std::collections::{HashMap, HashSet};

enum QRATRule {
    UnitPropagation,
    AddQRAT(Clause),
    ClauseRemoval(Clause),
    RemoveQRATLiteral(Clause, Literal),
}

impl QRATRule {
    pub fn apply(&self, mut formula: QBF) -> Option<QBF> {
        match self {
            QRATRule::UnitPropagation => {
                formula.cnf.implies_bot();
                return Some(formula);
            } // TODO rest of the rules
            QRATRule::AddQRAT(clause) => {
                formula.cnf.0.insert(clause.clone());
                if formula.is_qrat_clause(&clause) {
                    return Some(formula);
                }
                return None;
            }
            QRATRule::ClauseRemoval(clause) => {
                if formula.remove_clause(clause) {
                    return Some(formula);
                }
                return None;
            }
            QRATRule::RemoveQRATLiteral(clause, literal) => {
                if formula.is_qrat_literal(&clause, &literal) {
                    let QBF { cnf, vars } = formula;
                    let CNF(mut clauses) = cnf;
                    let Clause(mut literals) = clause.clone();

                    if clauses.remove(&clause) && literals.remove(&literal) {
                        clauses.insert(Clause(literals));
                        return Some(QBF {
                            cnf: CNF(clauses),
                            vars,
                        });
                    }
                }
                return None;
            }
        }
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
                new_literals.insert(l1.clone()); // TODO check for no assignment
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
