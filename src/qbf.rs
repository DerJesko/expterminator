use crate::literal::{Assignment, Literal};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

fn pop<T>(set: &mut HashSet<T>) -> Option<T>
where
    T: Eq + Clone + Hash,
{
    let top = set.iter().next().cloned();
    match &top {
        Some(t) => {
            set.remove(&t);
        }
        None => {}
    }
    top
}

#[macro_export]
macro_rules! h {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

#[derive(Clone, Debug)]
pub struct QBF {
    pub vars: Vec<usize>,
    pub cnf: CNF,
}

impl QBF {
    pub fn axiom(&self, clause: &Clause) -> Option<Clause> {
        let CNF(clauses) = &self.cnf;
        if !clauses.contains(clause) {
            return None;
        }
        let Clause(literals) = clause;
        let mut new_literals = HashSet::new();
        for l1 in literals.iter() {
            if l1.is_existential(&self.vars) {
                let mut map = HashMap::new();
                for l2 in literals.iter() {
                    if !l2.is_existential(&self.vars) && l2.less(l1, &self.vars) {
                        map.insert(l2.variable, !l2.positive);
                    }
                }
                let mut new_literal = l1.clone();
                new_literal.assignment = Assignment(map);
                new_literals.insert(new_literal); // TODO check for no assignment
            }
        }
        Some(Clause(new_literals))
    }

    pub fn resolution(
        &self,
        mut c1: Clause,
        l1: &Literal,
        mut c2: Clause,
        additional_clauses: &HashSet<Clause>,
    ) -> Option<Clause> {
        let CNF(clauses) = &self.cnf;

        {
            let Clause(literals1) = &mut c1;
            let Clause(literals2) = &mut c2;
            if !literals1.remove(&l1) || !literals2.remove(&l1.clone().invert()) {
                return None;
            }
        }
        if !(clauses.contains(&c1) || additional_clauses.contains(&c1))
            || !(clauses.contains(&c2) || additional_clauses.contains(&c2))
        {
            return None;
        }

        let Clause(mut literals1) = c1;
        let Clause(mut literals2) = c2;
        for l in literals2.drain() {
            literals1.insert(l);
        }
        Some(Clause(literals1))
    }

    pub fn depend_on(self, universal: Literal) -> HashSet<Literal> {
        let u = self
            .clone()
            .universal_possible_resolution_goal(&universal.clone());
        let u_inv = self.universal_possible_resolution_goal(&universal.invert());
        let mut result = HashSet::new();
        for l in u {
            if u_inv.contains(&l.clone().invert()) {
                result.insert(l.clone());
                result.insert(l.invert());
            }
        }
        result
    }

    pub fn universal_possible_resolution_goal(self, universal: &Literal) -> HashSet<Literal> {
        // TODO check shit & test
        let mut jumpoff_points = HashSet::new();
        let QBF { mut cnf, vars } = self;
        let CNF(clauses) = &cnf;
        let relevant_literal =
            |literal: &Literal| literal.is_existential(&vars) && universal.less(literal, &vars);
        for clause in clauses {
            let Clause(literals) = clause;
            if literals.contains(universal) {
                for e in literals.iter() {
                    if relevant_literal(e) {
                        jumpoff_points.insert(e.clone());
                    }
                }
            }
        }
        cnf.retain_literals(&relevant_literal);
        cnf.possible_resolution_goal(jumpoff_points)
    }
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
        let mut cnf = CNF(clauses);
        cnf.implies_bot()
    }

    pub fn is_qrat_literal(&self, clause: &Clause, literal: &Literal) -> bool {
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

    pub fn is_qrat_clause(&self, clause: &Clause) -> bool {
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

    pub fn remove_literal(&mut self, clause: Clause, literal: &Literal) -> bool {
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
    pub fn possible_resolution_goal(
        &self,
        mut jumpoff_points: HashSet<Literal>,
    ) -> HashSet<Literal> {
        let CNF(clauses) = self;
        let mut checked_jumpoff_points = HashSet::new();
        while let Some(mut check) = pop(&mut jumpoff_points) {
            check = check.invert();
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

    fn contains_unchecked_unit_clause(&self, checked_clauses: &HashSet<Clause>) -> Option<Clause> {
        let CNF(clauses) = self;
        for c in clauses {
            if c.is_unit() && !checked_clauses.contains(c) {
                return Some(c.clone());
            }
        }
        None
    }

    // TODO optimize
    // Apply unit propagation until it cannot be done anymore or an empty clause is reached
    pub fn implies_bot(&mut self) -> bool {
        let mut checked_clauses = HashSet::new();
        while let Some(clause) = self.contains_unchecked_unit_clause(&checked_clauses) {
            checked_clauses.insert(clause.clone());
            let Clause(literals) = clause;
            if let Some(literal) = literals.iter().next() {
                self.remove_literal_occurences(&(literal.clone().invert()));
                if self.contains_bot() {
                    return true;
                }
            } else {
                panic!("This should be impossible to reach");
            }
        }
        self.contains_bot()
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

    pub fn retain_literals<F>(&mut self, f: &F)
    where
        F: Fn(&Literal) -> bool,
    {
        let CNF(clauses) = self;
        let mut new_clauses = HashSet::new();
        for mut clause in clauses.drain() {
            clause.0.retain(f);
            new_clauses.insert(clause);
        }
        self.0 = new_clauses;
    }

    pub fn remove_literal_occurences(&mut self, literal: &Literal) {
        let CNF(clauses) = self;
        let mut new_clauses = HashSet::new();
        for mut clause in clauses.drain() {
            clause.0.remove(literal);
            new_clauses.insert(clause);
        }
        self.0 = new_clauses;
    }
}

impl PartialEq for CNF {
    fn eq(&self, other: &Self) -> bool {
        let CNF(s) = self;
        let CNF(o) = other;
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

impl Eq for CNF {}

#[derive(Clone, Debug)]
pub struct Clause(pub HashSet<Literal>);

impl Clause {
    fn is_unit(&self) -> bool {
        let Clause(literals) = self;
        literals.len() == 1
    }

    pub(crate) fn outer_resolvent(
        self,
        l: Literal,
        c2: Clause,
        vars: &Vec<usize>,
    ) -> Option<Clause> {
        let l_inv = l.clone().invert();
        let Clause(mut literals1) = self;
        let Clause(mut literals2) = c2;
        if !literals1.remove(&l) || !literals2.remove(&l_inv) {
            return None;
        }
        literals2.retain(|lit| lit.less_equal(&l, vars));
        for l in literals2 {
            literals1.insert(l);
        }
        Some(Clause(literals1))
    }

    pub fn hash_helper(&self) -> usize {
        let Clause(literals) = self;
        let mut accu = 1;
        for l in literals {
            accu *= (l.hash_helper() + 1);
        }
        accu
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
        self.hash_helper().hash(state);
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
