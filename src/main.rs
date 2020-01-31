#[macro_use]
mod qbf;
mod parse;
mod proof;

#[cfg(test)]
mod tests;

use crate::parse::{parse_proof, parse_qdimacs};
use crate::proof::{AllExpResRule, QRATRule};
use crate::qbf::{Clause, QBFLiteral};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut qbf_file_content = String::new();
    let mut proof_file_content = String::new();
    match std::env::args().nth(1) {
        Some(qbf_file) => {
            if qbf_file.starts_with("-") {
                print_help();
            }
            File::open(qbf_file)?.read_to_string(&mut qbf_file_content)?;
        }
        None => {
            print_help();
        }
    }
    match std::env::args().nth(2) {
        Some(proof_file) => {
            File::open(proof_file)?.read_to_string(&mut proof_file_content)?;
        }
        None => {
            io::stdin().read_to_string(&mut proof_file_content)?;
        }
    }
    let (clauses, orig_vars) = parse_qdimacs(qbf_file_content);
    let (new_vars, _, _, rule_applications) = parse_proof(proof_file_content);
    let mut qrat_rules = Vec::new();
    let mut remove_later = Vec::new();
    for i in 1..new_vars.len() {
        let var = new_vars[i];
        let var_annotated = orig_vars.len() + i - 1;
        qrat_rules.push(QRATRule::AddVariable(var, var_annotated));
        let a = Clause(
            [
                QBFLiteral(var as isize),
                QBFLiteral(-(var_annotated as isize)),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        let b = Clause(
            [
                QBFLiteral(-(var as isize)),
                QBFLiteral(var_annotated as isize),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        qrat_rules.push(QRATRule::AddClause(a.clone()));
        qrat_rules.push(QRATRule::AddClause(b.clone()));
        remove_later.push(a.clone());
        remove_later.push(b.clone());
    }
    let mut eliminate_universals = Vec::new();
    for rule in &rule_applications {
        match rule {
            AllExpResRule::Axiom(Clause(new_clause), previous_clause_index) => {
                let Clause(mut previous_clause) = clauses[*previous_clause_index - 1].clone();
                for QBFLiteral(l) in new_clause {
                    previous_clause.insert(QBFLiteral(*l).increase_var(orig_vars.len() - 1));
                    let l_without_annotation = new_vars[l.abs() as usize];
                    previous_clause.remove(&QBFLiteral(l_without_annotation as isize));
                    previous_clause.remove(&QBFLiteral(-(l_without_annotation as isize)));
                }
                eliminate_universals.push(Clause(previous_clause.clone()));
                qrat_rules.push(QRATRule::AddClause(Clause(previous_clause)));
            }
            AllExpResRule::Resolution(_, _, _) => {}
        }
    }
    for clause in clauses {
        qrat_rules.push(QRATRule::RemoveClause(clause));
    }
    for mut clause in eliminate_universals {
        while let Some(big_lit) = clause.find_biggest_universal(&orig_vars) {
            clause.0.remove(&big_lit);
            qrat_rules.push(QRATRule::RemoveLiteral(clause.clone(), big_lit.0));
        }
    }
    for rule in rule_applications {
        match rule {
            AllExpResRule::Axiom(_, _) => {}
            AllExpResRule::Resolution(Clause(mut literals), _, _) => {
                let mut new_literals = HashSet::new();
                for literal in literals.drain() {
                    new_literals.insert(literal.increase_var(orig_vars.len() - 1));
                }
                qrat_rules.push(QRATRule::AddClause(Clause(new_literals)));
            }
        }
    }
    for rule in qrat_rules {
        print!("{}", rule);
    }
    Ok(())
}

fn print_help() {
    println!("Help!");
    std::process::exit(0);
}
