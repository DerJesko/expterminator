extern crate ansi_term;

#[macro_use]
mod qbf;
mod literal;
mod parse;
//mod proof;

#[cfg(test)]
mod tests;

use crate::parse::{parse_proof, parse_qdimacs};
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
    let (_, qbf) = parse_qdimacs(qbf_file_content);
    println!("{}", qbf);
    //parse_proof(proof_file_content);
    Ok(())
}

fn print_help() {
    println!("Help!");
    std::process::exit(0);
}
