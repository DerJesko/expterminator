use ansi_term::Colour::RGB;
use std::collections::HashMap;
use std::fmt;

fn assignment_colour() -> ansi_term::Colour {
    RGB(100, 100, 100)
}

#[derive(Clone, Debug)]
pub struct QBF {
    pub vars: Vec<usize>,
    pub formula: CNF,
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
        s.push_str(&self.formula.to_string());
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct CNF(pub Vec<Clause>);

impl fmt::Display for CNF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let CNF(clauses) = self;
        let mut s = String::new();
        for i in 0..(clauses.len() as isize) - 1 {
            s.push_str(&clauses[i as usize].to_string());
            s.push_str("∨");
        }
        match clauses.last() {
            Some(c) => s.push_str(&c.to_string()),
            _ => {}
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Clause(pub Vec<Literal>);

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Clause(literals) = self;
        if literals.len() == 0 {
            return write!(f, "⊥");
        }
        let mut s = String::new();
        for i in 0..(literals.len() as isize) - 1 {
            s.push_str(&literals[i as usize].to_string());
            s.push_str("∧");
        }
        match literals.last() {
            Some(l) => s.push_str(&l.to_string()),
            _ => {}
        }
        write!(f, "({})", s)
    }
}

#[derive(Clone, Debug)]
pub struct LabeledVariable {
    pub variable: usize,
    pub assignment: Option<Assignment>,
}

impl fmt::Display for LabeledVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let LabeledVariable {
            variable,
            assignment,
        } = self;
        match assignment {
            Some(a) => write!(
                f,
                "{}[{}]",
                variable,
                assignment_colour().paint(a.to_string())
            ),
            None => write!(f, "{}", variable),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Positive(LabeledVariable),
    Negative(LabeledVariable),
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
