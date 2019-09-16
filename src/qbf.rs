use crate::literal::Literal;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

pub fn pop<T>(set: &mut HashSet<T>) -> T
where
    T: Eq + Clone + std::hash::Hash,
{
    let top = set.iter().next().cloned().unwrap();
    set.remove(&top);
    top
}

#[derive(Clone, Debug)]
pub struct QBF {
    pub vars: Vec<usize>,
    pub cnf: CNF,
}

impl QBF {
    fn quantifiers(&self) -> Vec<Vec<usize>> {
        let mut quantifiers: Vec<Vec<usize>> = vec![];
        for i in 0..self.vars.len() {
            for _ in 0..(self.vars[i] as isize) + 1 - (quantifiers.len() as isize) {
                quantifiers.push(vec![]);
            }
            quantifiers[self.vars[i]].push(i);
        }
        quantifiers
    }
}

impl QBF {
    // implies by unit propagation
    pub fn implies(self, clause: Clause) -> bool {
        let Clause(literals) = clause;
        let CNF(mut clauses) = self.cnf;
        for l in literals {
            if l.variable >= clauses.len() {
                return false;
            }
            let mut h = HashSet::new();
            h.insert(l.invert());
            clauses.insert(Clause(h));
        }
        CNF(clauses).implies_bot()
    }

    fn remove_universal_literals(&mut self) {
        let CNF(clauses) = &mut self.cnf;
        let mut new_clauses = HashSet::new();
        for mut clause in clauses.drain() {
            clause.remove_universal_literals(&self.vars);
            new_clauses.insert(clause);
        }
        self.cnf = CNF(new_clauses);
    }

    fn is_qrat_literal(&self, clause: &Clause, literal: &Literal) -> bool {
        let CNF(mut clauses) = self.cnf.clone();
        if !clauses.remove(clause) {
            return false;
        }
        // TODO Check shit

        for c in clauses {
            match clause
                .clone()
                .outer_resolvent(literal.clone(), c, &self.vars)
            {
                Some(resolvent) => {
                    if !self.clone().implies(resolvent) {
                        return false;
                    }
                }
                None => {}
            }
        }
        true
    }

    fn is_qrat_clause(&self, clause: &Clause) -> bool {
        // TODO Check shit
        let Clause(literals) = clause;
        for l in literals {
            if l.is_existential(&self.vars) && self.is_qrat_literal(clause, l) {
                return true;
            }
        }
        false
    }

    pub fn add_clause(&mut self, clause: &Clause) -> bool {
        let Clause(literals) = clause;
        for l in literals {
            if l.variable >= self.vars.len() {
                return false;
            }
        }
        let CNF(clauses) = &mut self.cnf;
        clauses.insert(clause.clone());
        true
    }
    pub fn remove_clause(&mut self, clause: &Clause) -> bool {
        self.cnf.remove_clause(clause)
    }

    pub fn remove_literal(&mut self, mut clause: Clause, literal: &Literal) -> bool {
        self.cnf.remove_literal(clause, literal)
    }
}

impl fmt::Display for QBF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let quantifiers = self.quantifiers();
        let mut s = String::new();
        for i in 0..quantifiers.len() {
            if quantifiers[i].len() > 0 {
                if i % 2 == 0 {
                    s.push_str("∃");
                } else {
                    s.push_str("∀");
                }
                for j in 0..(quantifiers[i].len() as isize) - 1 {
                    s.push_str(&quantifiers[i][j as usize].to_string());
                    s.push_str(",");
                }
                match quantifiers[i].last() {
                    Some(v) => s.push_str(&v.to_string()),
                    _ => {}
                }
            }
        }
        s.push_str(":");
        s.push_str(&self.cnf.to_string());
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct CNF(pub HashSet<Clause>);

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let CNF(clauses) = self;
        let mut s = String::new();
        for (i, c) in clauses.iter().enumerate() {
            s.push_str(&c.to_string());
            if i < clauses.len() - 1 {
                s.push_str("∨");
            }
        }
        write!(f, "{}", s)
    }
}

impl CNF {
    fn possible_resolution_goal(&self, mut jumpoff_points: HashSet<Literal>) -> HashSet<Literal> {
        let CNF(clauses) = self;
        let mut checked_jumpoff_points = HashSet::new();
        while !jumpoff_points.is_empty() {
            let check = pop(&mut jumpoff_points).invert();
            for clause in clauses {
                let Clause(literals) = clause;
                if literals.contains(&check) {
                    for literal in literals {
                        if !literal.var_eq(&check) && !checked_jumpoff_points.contains(literal) {
                            jumpoff_points.insert(literal.clone());
                        }
                    }
                }
            }
            checked_jumpoff_points.insert(check.invert());
        }
        checked_jumpoff_points
    }
    fn contains_bot(&self) -> bool {
        let CNF(clauses) = self;
        clauses.contains(&Clause(HashSet::new()))
    }

    fn remove_clauses_containing(&mut self, literal: &Literal) {
        let CNF(clauses) = self;
        clauses.retain(|clause| !clause.0.contains(literal));
    }

    fn contains_unit_clause(&self) -> Option<Clause> {
        let CNF(clauses) = self;
        for c in clauses {
            if c.is_unit() {
                return Some(c.clone());
            }
        }
        None
    }

    // Apply unit propagation until it cannot be done anymore or an empty clause is reached
    pub fn implies_bot(&mut self) -> bool {
        while let Some(clause) = self.contains_unit_clause() {
            let Clause(literals) = clause;
            if let Some(literal) = literals.iter().next() {
                self.remove_clauses_containing(literal);
                self.remove_literals(literal);
                if self.contains_bot() {
                    return true;
                }
            } else {
                println!("This should be impossible to reach");
            }
        }
        false
    }

    pub fn remove_clause(&mut self, clause: &Clause) -> bool {
        let CNF(clauses) = self;
        clauses.remove(clause)
    }

    pub fn remove_literal(&mut self, mut clause: Clause, literal: &Literal) -> bool {
        {
            let Clause(literals) = &clause;
            if !literals.contains(literal) {
                return false;
            }
            if !self.remove_clause(&clause) {
                return false;
            }
        }
        let Clause(literals) = &mut clause;
        if !literals.remove(literal) {
            panic!("This should be impossible");
        }
        let CNF(clauses) = self;
        clauses.insert(clause);
        true
    }

    pub fn remove_literals(&mut self, literal: &Literal) {
        let CNF(clauses) = self;
        let mut new_clauses = HashSet::new();
        for mut clause in clauses.drain() {
            clause.0.retain(|lit| lit == literal);
            new_clauses.insert(clause);
        }
        self.0 = new_clauses;
    }
}

#[derive(Clone, Debug)]
pub struct Clause(pub HashSet<Literal>);

impl Clause {
    fn is_unit(&self) -> bool {
        let Clause(literals) = self;
        literals.len() == 1
    }

    fn remove_universal_literals(&mut self, vars: &Vec<usize>) {
        let Clause(literals) = self;
        let mut new_literals = HashSet::new();
        for literal in literals.drain() {
            if literal.is_existential(vars) {
                new_literals.insert(literal);
            }
        }
        self.0 = new_literals;
    }

    fn outer_resolvent(self, l: Literal, c2: Clause, vars: &Vec<usize>) -> Option<Clause> {
        let l_inv = l.clone().invert();
        let Clause(mut literals1) = self;
        let Clause(mut literals2) = c2;
        if !literals1.remove(&l) || !literals2.remove(&l_inv) {
            return None;
        }
        literals2.retain(|lit| lit.less_equal(&l, vars));
        literals1.union(&literals2);
        Some(Clause(literals1))
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Clause(literals) = self;
        if literals.is_empty() {
            return write!(f, "⊥");
        }
        let mut s = String::new();
        for (i, l) in literals.iter().enumerate() {
            s.push_str(&l.to_string());
            if i < literals.len() - 1 {
                s.push_str("∧");
            }
        }
        write!(f, "({})", s)
    }
}

impl Hash for Clause {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Clause(literals) = self;
        for i in literals.iter() {
            i.hash(state);
        }
    }
}

impl PartialEq for Clause {
    fn eq(&self, other: &Self) -> bool {
        let Clause(s) = self;
        let Clause(o) = other;
        if s.len() != o.len() {
            return false;
        }
        for i in s.iter() {
            if !o.contains(i) {
                return false;
            }
        }
        true
    }
}

impl Eq for Clause {}
