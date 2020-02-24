use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

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

pub fn universal_literals(vars: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for var in 0..vars.len() {
        let block = vars[var];
        if block % 2 == 1 {
            result.push((block, var))
        }
    }
    result.sort_by(|(a1, _), (b1, _)| b1.cmp(a1));
    result
}

#[derive(Clone, Debug)]
pub struct Clause(pub HashSet<QBFLiteral>);

impl Clause {
    pub fn hash_helper(&self) -> usize {
        let Clause(literals) = self;
        let mut accu = 1;
        for l in literals {
            accu = (accu * (l.hash_helper() + 7)) % 2147483647;
        }
        accu
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Clause(literals) = self;
        let mut s = String::new();
        for l in literals.iter() {
            s.push_str(&l.to_string());
            s.push_str(" ");
        }
        write!(f, "{}", s)
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QBFLiteral(pub isize);

impl QBFLiteral {
    pub fn variable(&self) -> usize {
        self.0.abs() as usize
    }

    pub fn hash_helper(&self) -> usize {
        self.variable() as usize * 2 + if self.0 > 0 { 1 } else { 0 }
    }

    pub fn increase_var(&self, a: usize) -> Self {
        if self.0 < 0 {
            QBFLiteral(self.0 - (a as isize))
        } else {
            QBFLiteral(self.0 + (a as isize))
        }
    }
}

impl fmt::Display for QBFLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
