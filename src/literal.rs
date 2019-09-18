use ansi_term::Colour::RGB;
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

    pub fn less(&self, other: &Literal, vars: &Vec<usize>) -> bool {
        vars[self.variable] < vars[other.variable]
    }

    pub fn less_equal(&self, other: &Literal, vars: &Vec<usize>) -> bool {
        vars[self.variable] <= vars[other.variable]
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
