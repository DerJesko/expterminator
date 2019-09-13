use crate::qbf::{Assignment, Clause, LabeledVariable, Literal, CNF, QBF};
use std::collections::HashMap;

#[test]
fn display() {
    let lit1 = Literal::Positive(LabeledVariable {
        variable: 0,
        assignment: None,
    });
    let mut map = HashMap::new();
    map.insert(2, true);
    map.insert(24, false);
    let lit2 = Literal::Negative(LabeledVariable {
        variable: 3,
        assignment: Some(Assignment(map)),
    });
    /*
    let qbf = QBF {
        vars: vec![1, 2, 4, 2],
        cnf: CNF(vec![Clause(vec![lit1, lit2]), Clause(vec![])]),
    };
    println!("{}", qbf);
    */
}
