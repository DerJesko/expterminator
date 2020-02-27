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
    // Read the QBF file
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
    // Read the proof file
    match std::env::args().nth(2) {
        Some(proof_file) => {
            File::open(proof_file)?.read_to_string(&mut proof_file_content)?;
        }
        None => {
            io::stdin().read_to_string(&mut proof_file_content)?;
        }
    }
    // Parse the two read files
    let (clauses, orig_vars) = parse_qdimacs(qbf_file_content);
    let (new_vars, rule_applications) = parse_proof(proof_file_content);
    // Step 1
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
    // Step 2
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
    // Step 3
    // Remove original clauses
    for clause in clauses {
        qrat_rules.push(QRATRule::RemoveClause(clause));
    }
    // Remove the clauses introduced in step 1
    for clause in remove_later {
        qrat_rules.push(QRATRule::RemoveClause(clause));
    }
    // Step 4
    for (_, literal) in qbf::universal_literals(&orig_vars) {
        //biggest to smallest
        let mut temp_eliminate_universals = Vec::new();
        for mut clause in eliminate_universals.drain(..) {
            if clause.0.remove(&QBFLiteral(literal as isize)) {
                qrat_rules.push(QRATRule::RemoveLiteral(clause.clone(), literal as isize));
            }
            if clause.0.remove(&QBFLiteral(-(literal as isize))) {
                qrat_rules.push(QRATRule::RemoveLiteral(clause.clone(), -(literal as isize)));
            }
            temp_eliminate_universals.push(clause);
        }
        eliminate_universals = temp_eliminate_universals;
    }
    // Step 5
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
    println!("usage: ./expterminator INPUT_FORMULA_PATH [INPUT_PROOF_PATH]```

    * `INPUT_FORMULA_PATH` is the path to the QDIMACS file containing a quantified boolean formula.
    * `INPUT_PROOF_PATH` is the path to the ∀-Exp+Res proof file.
    If this is not specified the tool expects the proof to be input via STDIN.
    * expterminator outputs the QRAT proof on STDOUT.

For example, if your formula is in the QDIMACS file 'formula.qdimacs' (located in the directory from which you call expterminator) and your ∀-Exp+Res proof is in the file 'proof.res', then the following command writes its output to STDOUT:
./drat2er formula.qdimacs proof.res");
    std::process::exit(0);
}
