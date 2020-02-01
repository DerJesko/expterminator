use crate::proof::AllExpResRule;
use crate::qbf::{Clause, QBFLiteral};
use std::collections::HashSet;

static PREAMBLE_BROKEN: &str = "the preamble of the qdimacs file is broken";
static QUANTIFIER_BROKEN: &str = "the quantifier part of the qdimacs file is broken";
static CLAUSE_BROKEN: &str = "the clause matrix of the qdimacs file is broken";

pub fn parse_qdimacs(qdimacs_string: String) -> (Vec<Clause>, Vec<usize>) {
    let mut lines = qdimacs_string.lines().peekable();
    // Comment Lines
    while lines.peek().expect(PREAMBLE_BROKEN).starts_with("c") {
        lines.next();
    }
    // Problem Line
    let mut problem_line = lines
        .next()
        .expect(PREAMBLE_BROKEN)
        .split_whitespace()
        .peekable();
    if problem_line.next().expect(PREAMBLE_BROKEN) != "p" {
        panic!(PREAMBLE_BROKEN);
    }
    if problem_line.next().expect(PREAMBLE_BROKEN) != "cnf" {
        panic!(PREAMBLE_BROKEN);
    }
    let max_var = problem_line
        .next()
        .expect(PREAMBLE_BROKEN)
        .parse::<usize>()
        .expect(PREAMBLE_BROKEN);

    let clause_number = problem_line
        .next()
        .expect(PREAMBLE_BROKEN)
        .parse::<usize>()
        .expect(PREAMBLE_BROKEN);
    // Quantifier Lines
    let mut vars = Vec::with_capacity(max_var + 1);
    for _ in 0..max_var + 1 {
        vars.push(0);
    }
    let mut was_existential = false; // This implicitly make sure the "0" quantifier block is existstential
    let mut i = 0;
    while lines.peek().expect(QUANTIFIER_BROKEN).starts_with("e")
        | lines.peek().expect(QUANTIFIER_BROKEN).starts_with("a")
    {
        let mut literals = lines
            .next()
            .expect(QUANTIFIER_BROKEN)
            .split_whitespace()
            .peekable();
        let is_existential = literals.next().expect(QUANTIFIER_BROKEN) == "e";
        if was_existential && is_existential || !was_existential && !is_existential {
            i += 1;
        }
        while literals.peek().expect(QUANTIFIER_BROKEN) != &"0" {
            let literal = literals
                .next()
                .expect(QUANTIFIER_BROKEN)
                .parse::<usize>()
                .expect(QUANTIFIER_BROKEN);
            vars[literal] = i;
            was_existential = is_existential;
        }
        i += 1;
    }
    // Clause Lines
    let mut clauses = Vec::new();
    for _ in 0..clause_number {
        let mut clause = HashSet::new();
        let mut literals = lines
            .next()
            .expect(CLAUSE_BROKEN)
            .split_whitespace()
            .peekable();
        while literals.peek().expect(CLAUSE_BROKEN) != &"0" {
            let literal = QBFLiteral(
                literals
                    .next()
                    .expect(CLAUSE_BROKEN)
                    .parse::<isize>()
                    .expect(CLAUSE_BROKEN),
            );
            clause.insert(literal);
        }
        let c = Clause(clause);
        clauses.push(c);
    }
    (clauses, vars)
}

static ANNOTATIONS_BROKEN: &str = "the annotations of the proof file are broken";
static RULE_BROKEN: &str = "the rule application part of the proof file is broken";

pub fn parse_proof(qdimacs_string: String) -> (Vec<usize>, Vec<AllExpResRule>) {
    let mut lines = qdimacs_string.lines().peekable();
    let mut qbf_vars = Vec::new();
    qbf_vars.push(0);
    // Annotation Lines
    while lines.peek().expect(ANNOTATIONS_BROKEN).starts_with("x") {
        let mut words = lines
            .next()
            .expect(ANNOTATIONS_BROKEN)
            .split_whitespace()
            .peekable();
        words.next().expect(ANNOTATIONS_BROKEN);
        while words.peek().expect(ANNOTATIONS_BROKEN) != &"0" {
            words
                .next()
                .expect(ANNOTATIONS_BROKEN)
                .parse::<usize>()
                .expect(ANNOTATIONS_BROKEN);
        }
        words.next().expect(ANNOTATIONS_BROKEN);
        while words.peek().expect(ANNOTATIONS_BROKEN) != &"0" {
            qbf_vars.push(
                words
                    .next()
                    .expect(ANNOTATIONS_BROKEN)
                    .parse::<usize>()
                    .expect(ANNOTATIONS_BROKEN),
            );
        }
        // No need to actually parse the annotations
    }
    // Rule Lines
    let mut rule_applications = Vec::new();
    while lines.peek().is_some() {
        let mut words = lines
            .next()
            .expect(RULE_BROKEN)
            .split_whitespace()
            .peekable();
        let _ = words
            .next()
            .expect(RULE_BROKEN)
            .parse::<usize>()
            .expect(RULE_BROKEN);
        let mut literals = HashSet::new();
        while words.peek().expect(RULE_BROKEN) != &"0" {
            literals.insert(QBFLiteral(
                words
                    .next()
                    .expect(RULE_BROKEN)
                    .parse::<isize>()
                    .expect(RULE_BROKEN),
            ));
        }
        words.next().expect(RULE_BROKEN);
        let antecedent1 = words
            .next()
            .expect(RULE_BROKEN)
            .parse::<usize>()
            .expect(RULE_BROKEN);
        let antecedent2 = words
            .next()
            .expect(RULE_BROKEN)
            .parse::<usize>()
            .expect(RULE_BROKEN);
        if antecedent2 == 0 {
            rule_applications.push(AllExpResRule::Axiom(Clause(literals), antecedent1));
        } else {
            rule_applications.push(AllExpResRule::Resolution(
                Clause(literals),
                antecedent1,
                antecedent2,
            ));
        }
    }
    (qbf_vars, rule_applications)
}
