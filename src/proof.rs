use crate::literal::{Assignment, Literal};
use crate::qbf::{Clause, CNF, QBF};
use std::collections::{HashMap, HashSet};

struct QRATProof(Vec<QRATRule>);

enum QRATRule {
    UnitPropagation,
    AddQRAT(Clause),
    ClauseRemoval(Clause),
    RemoveQRATLiteral(Clause, Literal),
    ExtendedUniversalReduction(Clause, Literal),
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
            _ => None,
        }
    }
}

struct AllExpResProof {
    rules: Vec<AllExpResRule>,
    initial_formula: QBF,
}
impl AllExpResProof {
    pub fn to_qrat(self) -> Option<QRATProof> {
        let mut result = Vec::new();
        let mut step_3 = Vec::new();
        let rules = &self.rules;
        // Step 1
        for r in rules {
            match r {
                AllExpResRule::Axiom(clause) => match self.initial_formula.axiom(clause) {
                    Some(new_clause) => {
                        let Clause(literals) = new_clause;
                        for l in literals {
                            if !l.is_pure() {
                                let c1 = Clause(h![l.clone().purify(), l.clone().invert()]);
                                result.push(QRATRule::AddQRAT(c1.clone()));
                                step_3.push(QRATRule::ClauseRemoval(c1));
                                let c2 = Clause(h![l.clone(), l.clone().invert().purify()]);
                                result.push(QRATRule::AddQRAT(c2.clone()));
                                step_3.push(QRATRule::ClauseRemoval(c2));
                            }
                        }
                    }
                    None => {
                        return None;
                    }
                },
                AllExpResRule::Resolution(_, _, _) => {}
            }
        }
        // Step 2
        let mut step_4_literals = Vec::new();
        let mut step_4_cnf = HashSet::new();
        for r in rules {
            match r {
                AllExpResRule::Axiom(clause) => match self.initial_formula.axiom(clause) {
                    Some(new_clause) => {
                        let Clause(mut new_literals) = new_clause;
                        let Clause(literals) = clause;
                        for l in literals {
                            if !l.is_existential(&self.initial_formula.vars) {
                                new_literals.insert(l.clone());
                                step_4_literals.push(l);
                            }
                        }
                        result.push(QRATRule::AddQRAT(Clause(new_literals.clone())));
                        step_4_cnf.insert(Clause(new_literals));
                    }
                    _ => {
                        return None;
                    }
                },
                AllExpResRule::Resolution(_, _, _) => {}
            }
        }
        // Step 3
        for removal in step_3 {
            result.push(removal)
        }
        for clause in &self.initial_formula.cnf.0 {
            result.push(QRATRule::ClauseRemoval(clause.clone()));
        }
        // Step 4
        step_4_literals.sort_by(|a, b| a.cmp_inv(b, &self.initial_formula.vars));
        let mut already_removed = HashSet::new();
        for l in step_4_literals {
            if !already_removed.contains(l) {
                for clause in &step_4_cnf {
                    let Clause(literals) = clause;
                    if literals.contains(&l) {
                        result.push(QRATRule::ExtendedUniversalReduction(
                            clause.clone(),
                            l.clone(),
                        ))
                    }
                }
                already_removed.insert(l);
            }
        }
        // Step 5
        for r in rules {
            match r {
                AllExpResRule::Resolution(clause1, literal, clause2) => {
                    match self.initial_formula.resolution(
                        clause1.clone(),
                        literal,
                        clause2.clone(),
                        &step_4_cnf, // TODO this has to be modified before this step
                    ) {
                        Some(clause) => result.push(QRATRule::AddQRAT(clause)),
                        None => {
                            return None;
                        }
                    }
                }
                AllExpResRule::Axiom(_) => {}
            }
        }
        Some(QRATProof(result))
    }
    pub fn check(&self, mut formula: QBF) -> bool {
        false // TODO
    }
}
pub enum AllExpResRule {
    Axiom(Clause),                       // Clause used for the axiom rule
    Resolution(Clause, Literal, Clause), // (Clause, Literal) pairs used for resolution
}

impl AllExpResRule {
    pub fn apply(&self, formula: &QBF, additional_clauses: &HashSet<Clause>) -> Option<Clause> {
        match self {
            AllExpResRule::Axiom(clause) => formula.axiom(&clause),
            AllExpResRule::Resolution(c1, l1, c2) => {
                formula.resolution(c1.clone(), l1, c2.clone(), additional_clauses)
            }
        }
    }
}
