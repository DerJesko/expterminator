use crate::qbf::Clause;
use std::fmt;

#[derive(Debug)]
pub enum AllExpResRule {
    Axiom(Clause, usize),             // New clause and antecedent
    Resolution(Clause, usize, usize), // New clause and two anthecedents
}

#[derive(Debug)]
pub enum QRATRule {
    AddVariable(usize, usize), // Variable which specifies the block, new variable
    AddClause(Clause),
    RemoveClause(Clause),
    RemoveLiteral(Clause, isize), // Clause with literal already removed, removed literal
}

impl fmt::Display for QRATRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QRATRule::AddVariable(old, new) => writeln!(f, "a {} {} 0", old, new),
            QRATRule::RemoveClause(clause) => writeln!(f, "d {}0", clause),
            QRATRule::AddClause(clause) => writeln!(f, "{}0", clause),
            QRATRule::RemoveLiteral(clause, literal) => {
                writeln!(f, "u {} {}0", literal, clause) //TODO
            }
        }
    }
}
