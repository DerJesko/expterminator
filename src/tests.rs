use crate::literal::{Assignment, Literal};
use crate::qbf::{Clause, CNF, QBF};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

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

fn create4() -> QBF {
    let a = Literal {
        positive: true,
        variable: 0,
        assignment: Assignment(HashMap::new()),
    };
    let b = Literal {
        positive: true,
        variable: 1,
        assignment: Assignment(HashMap::new()),
    };
    let x = Literal {
        positive: true,
        variable: 2,
        assignment: Assignment(HashMap::new()),
    };
    let y = Literal {
        positive: true,
        variable: 3,
        assignment: Assignment(HashMap::new()),
    };
    let c = Literal {
        positive: true,
        variable: 4,
        assignment: Assignment(HashMap::new()),
    };
    QBF {
        vars: vec![0, 0, 1, 1, 2],
        cnf: CNF(h![
            Clause(h![b.clone().invert(), y.clone().invert(), c.clone()]),
            Clause(h![a.clone(), y.clone().invert(), c.clone()]),
            Clause(h![a.clone(), b.clone(), x.clone()]),
            Clause(h![b, x, y])
        ]),
    }
}

#[test]
fn display() {
    println!("{}", create1());
    println!("{}", create2());
    println!("{}", create3());
    println!("{}", create4());
}

#[test]
fn implies_bot() {
    assert!(create1().cnf.implies_bot());
    assert!(create2().cnf.implies_bot());
    assert!(!create3().cnf.implies_bot());
}

#[test]
fn clause_eq() {
    let a = Literal {
        positive: true,
        variable: 0,
        assignment: Assignment(HashMap::new()),
    };
    let b = Literal {
        positive: true,
        variable: 1,
        assignment: Assignment(HashMap::new()),
    };
    assert_eq!(
        Clause(h![a.clone(), b.clone()]),
        Clause(h![a.clone(), b.clone()])
    );
    assert_eq!(
        Clause(h![a.clone(), b.clone()]),
        Clause(h![b.clone(), a.clone()])
    );
    assert!(Clause(h![a.clone(), b.clone()]) != Clause(h![a.clone()]));
}

#[test]
fn clause_hash() {
    let a = Literal {
        positive: true,
        variable: 0,
        assignment: Assignment(HashMap::new()),
    };
    let b = Literal {
        positive: true,
        variable: 1,
        assignment: Assignment(HashMap::new()),
    };
    let mut h1 = DefaultHasher::new();
    let c1 = Clause(h![a.clone(), b.clone()]);
    c1.hash(&mut h1);
    println!("h1:{}", c1.hash_helper());
    let mut h2 = DefaultHasher::new();
    let c2 = Clause(h![b.clone(), a.clone()]);
    c2.hash(&mut h2);
    println!("h2:{}", c2.hash_helper());
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn qrat_literal() {
    let b = Literal {
        positive: true,
        variable: 1,
        assignment: Assignment(HashMap::new()),
    };
    let x = Literal {
        positive: true,
        variable: 2,
        assignment: Assignment(HashMap::new()),
    };
    let y = Literal {
        positive: true,
        variable: 3,
        assignment: Assignment(HashMap::new()),
    };
    let qbf = create4();
    println!("qbf: {}", qbf);
    let clause = Clause(h![b, x, y.clone()]);
    let literal = y;
    assert!(qbf.is_qrat_literal(&clause, &literal));
}
