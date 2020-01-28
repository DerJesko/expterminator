use crate::qbf::{Clause, CNF, QBF};
use std::collections::{HashMap, HashSet};

pub enum AllExpResRule {
    Axiom(Clause, usize),             // New clause and antecedent
    Resolution(Clause, usize, usize), // New clause and two anthecedents
}
