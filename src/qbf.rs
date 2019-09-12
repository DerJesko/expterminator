use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub(crate) struct QBF {
    vars: Vec<usize>,
    formula: CNF,
}

#[derive(Clone, Debug)]
struct CNF {
    clauses: Vec<Clause>,
}

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.clauses.len() - 1 {
            s.push_str(&self.clauses[i].to_string());
            s.push_str("∨");
        }
        if self.clauses.len() != 0 {
            s.push_str(&self.clauses[self.clauses.len()].to_string());
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Clause {
    literals: Vec<Literal>,
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.literals.len() - 1 {
            s.push_str(&self.literals[i].to_string());
            s.push_str("∧");
        }
        if self.literals.len() != 0 {
            s.push_str(&self.literals[self.literals.len()].to_string());
        }
        write!(f, "({})", s)
    }
}

#[derive(Clone, Debug)]
struct LabeledVariable {
    basic_variable: usize,
    quantified_at: usize,
    assignment: Assignment,
}

#[derive(Clone, Debug)]
enum Literal {
    Positive(usize),
    Negative(usize),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Positive(x) => write!(f, "{}", x),
            Literal::Negative(x) => write!(f, "¬{}", x),
        }
    }
}

#[derive(Clone, Debug)]
struct Assignment {
    map: HashMap<usize, bool>,
}

impl Assignment {
    // If a key is in both self an other, then the one in self will be chosen
    pub(crate) fn compose(&mut self, other: &Assignment) {
        for (key, val) in other.map.iter() {
            self.map.entry(*key).or_insert(*val);
        }
    }
}

impl PartialEq for Assignment {
    fn eq(&self, other: &Self) -> bool {
        if self.map.keys().len() != other.map.keys().len() {
            return false;
        }
        for (key, val) in self.map.iter() {
            match other.map.get(key) {
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
