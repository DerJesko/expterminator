use crate::literal::{Assignment, Literal};
use crate::qbf::{Clause, CNF, QBF};
use std::collections::{HashMap, HashSet};

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

fn create1() -> QBF {
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
        cnf: CNF(h![Clause(h![lit1, lit2]), Clause(h![])]),
    };
    qbf
}

fn create2() -> QBF {
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
        cnf: CNF(h![
            Clause(h![lit1.clone(), lit2.clone()]),
            Clause(h![lit1.invert()]),
            Clause(h![lit2.invert()])
        ]),
    };
    qbf
}

fn create3() -> QBF {
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
        cnf: CNF(h![
            Clause(h![lit1.clone(), lit2.clone()]),
            Clause(h![lit1]),
            Clause(h![lit2.invert()])
        ]),
    };
    qbf
}

#[test]
fn display() {
    println!("{}", create1());
    println!("{}", create2());
    println!("{}", create3());
}

#[test]
fn implies_bot() {
    assert!(create1().cnf.implies_bot());
    assert!(create2().cnf.implies_bot());
    assert!(!create3().cnf.implies_bot());
}
