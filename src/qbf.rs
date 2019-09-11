use std::collections::HashMap;

struct QBF {
    vars: Vec<usize>,
    formula: CNF,
}

struct CNF {
    clauses: Vec<Clause>,
}

pub(crate) struct Clause {
    literals: Vec<Literal>,
}

struct LabeledVariable {
    basic_variable: usize,
    quantified_at: usize,
    assignment: Assignment,
}

enum Literal {
    Positive(usize),
    Negative(usize),
}

struct Assignment {
    map: HashMap<usize, bool>,
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
