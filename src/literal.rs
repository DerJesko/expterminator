use ansi_term::Colour::RGB;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

fn assignment_colour() -> ansi_term::Colour {
    RGB(100, 100, 100)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Literal {
    pub positive: bool,
    pub variable: usize,
    pub assignment: Assignment,
}

impl Literal {
    pub fn var_eq(&self, other: &Literal) -> bool {
        self.variable == other.variable && self.assignment == other.assignment
    }
    pub fn is_existential(&self, vars: &Vec<usize>) -> bool {
        vars[self.variable] % 2 == 0
    }

    pub fn cmp(&self, other: &Literal, vars: &Vec<usize>) -> Ordering {
        vars[self.variable].cmp(&vars[other.variable])
    }

    pub fn less(&self, other: &Literal, vars: &Vec<usize>) -> bool {
        match self.cmp(other, vars) {
            Ordering::Less => true,
            _ => false,
        }
    }

    pub fn less_equal(&self, other: &Literal, vars: &Vec<usize>) -> bool {
        match self.cmp(other, vars) {
            Ordering::Greater => false,
            _ => true,
        }
    }

    pub fn is_inverse(&self, other: &Literal) -> bool {
        self.positive != other.positive
            && self.assignment == other.assignment
            && self.variable == other.variable
    }

    pub fn invert(mut self) -> Literal {
        self.positive = !self.positive;
        self
    }

    pub fn purify(mut self) -> Literal {
        self.assignment = Assignment(HashMap::new());
        self
    }

    pub fn is_pure(&self) -> bool {
        self.assignment.0.len() == 0
    }

    pub(crate) fn hash_helper(&self) -> usize {
        if self.positive {
            (self.assignment.hash_helper() + 1) * self.variable
        } else {
            (self.assignment.hash_helper() + 1) * self.variable + 1
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if !self.positive {
            s.push_str("¬");
        }
        if self.assignment.0.is_empty() {
            write!(f, "{}{}", s, self.variable)
        } else {
            write!(
                f,
                "{}{}[{}]",
                s,
                self.variable,
                assignment_colour().paint(self.assignment.to_string())
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assignment(pub HashMap<usize, bool>);

impl Assignment {
    fn hash_helper(&self) -> usize {
        let Assignment(map) = self;
        let mut accu = 0;
        for (key, val) in map {
            if *val {
                accu += 2 * key + 1;
            } else {
                accu += 2 * key;
            }
        }
        accu
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
        self.hash_helper().hash(state);
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
                    if val != other_val {
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
