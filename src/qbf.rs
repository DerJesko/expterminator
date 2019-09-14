use ansi_term::Colour::RGB;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

fn assignment_colour() -> ansi_term::Colour {
    RGB(100, 100, 100)
}

#[derive(Clone, Debug)]
pub struct QBF {
    pub vars: Vec<usize>,
    pub cnf: CNF,
}

impl QBF {
    fn quantifiers(&self) -> Vec<Vec<usize>> {
        let mut quantifiers: Vec<Vec<usize>> = vec![];
        println!("q: {:?}", quantifiers);
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
        let CNF(clauses) = &mut self.cnf;
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
        let CNF(clauses) = &mut self.cnf;
        clauses.insert(clause);
        true
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
            //TODO make into function or macro ???
            s.push_str(&c.to_string());
            if i < clauses.len() - 1 {
                s.push_str("∨");
            }
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Clause(pub HashSet<Literal>);

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Clause(literals) = self;
        if literals.len() == 0 {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Literal {
    positive: bool,
    pub variable: usize,
    pub assignment: Option<Assignment>,
}

impl Literal {
    pub fn is_existential(&self, vars: &Vec<usize>) -> bool {
        vars[self.variable] % 2 == 0
    }

    pub fn less(&self, other: &Literal, vars: &Vec<usize>) -> bool {
        vars[self.variable] < vars[other.variable]
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if !self.positive {
            s.push_str("¬");
        }
        match &self.assignment {
            Some(a) => write!(
                f,
                "{}{}[{}]",
                s,
                self.variable,
                assignment_colour().paint(a.to_string())
            ),
            None => write!(f, "{}{}", s, self.variable),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assignment(pub HashMap<usize, bool>);

impl Assignment {
    // If a key is in both self an other, then the one in self will be chosen
    pub fn compose(&mut self, other: &Assignment) {
        let Assignment(map_s) = self;
        let Assignment(map_o) = other;
        for (key, val) in map_o.iter() {
            map_s.entry(*key).or_insert(*val);
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Assignment(map) = self;
        let mut s = String::new();
        for (i, (key, val)) in map.iter().enumerate() {
            if !*val {
                s.push_str("¬");
            }
            s.push_str(&key.to_string());
            if i != map.len() - 1 {
                s.push_str(",");
            }
        }
        write!(f, "{}", s)
    }
}

impl Hash for Assignment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Assignment(map) = self;
        for (key, val) in map.iter() {
            key.hash(state);
            val.hash(state);
        }
    }
}

impl PartialEq for Assignment {
    fn eq(&self, other: &Self) -> bool {
        let Assignment(map_s) = self;
        let Assignment(map_o) = other;
        if map_s.keys().len() != map_o.keys().len() {
            return false;
        }
        for (key, val) in map_s.iter() {
            match map_o.get(key) {
                Some(other_val) => {
                    if val == other_val {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Eq for Assignment {}
