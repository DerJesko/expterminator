use crate::literal::{Assignment, Literal};
use crate::qbf::{Clause, CNF, QBF};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// vector to hash
fn v2h<T>(mut v: Vec<T>) -> HashSet<T>
where
    T: Hash + Eq,
{
    v.drain(0..).collect()
}

#[test]
fn display() {
    let lit1 = Literal {
        positive: true,
        variable: 0,
        assignment: Assignment(HashMap::new()),
    };
    let mut map = HashMap::new();
    map.insert(2, true);
    map.insert(24, false);
    let lit2 = Literal {
        positive: false,
        variable: 3,
        assignment: Assignment(map),
    };
    let qbf = QBF {
        vars: vec![1, 2, 4, 2],
        cnf: CNF(v2h(vec![
            Clause(v2h(vec![lit1, lit2])),
            Clause(v2h(vec![])),
        ])),
    };
    println!("{}", qbf);
}
