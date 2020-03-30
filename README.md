# expterminator: Proof Transformer from ∀-Exp+Res to QRAT

expterminator is a tool for transforming proofs that are produced by solvers of [quantified Boolean formulas](https://en.wikipedia.org/wiki/True_quantified_Boolean_formula). These solvers play an important role in computer science because they can be used as reasoning engines for the verification of software and hardware systems as well as for various tasks in artificial intelligence. In particular, expterminator can transform [∀-Exp+Res proofs](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.705.1452&rep=rep1&type=pdf) into [QRAT proofs](https://www.cs.utexas.edu/~marijn/publications/jar16.pdf). The details of this proof transformation are described in the paper [QRAT Polynomially Simulates ∀-Exp+Res](https://benjaminkiesl.github.io/publications/qrat_simulates_forall_exp_res_kiesl_seidl.pdf).

## Getting Started

### Prerequistites

To build expterminator, you need to have [Cargo](https://github.com/rust-lang/cargo/) and with a working [Rust toolchain](https://github.com/rust-lang/rust).

### Installation

The easiest way to build expterminator is to run `cargo build --release` in the main directory. After this, the executable 'expterminator' is located at `target/release/expterminator` in the project folder.

### Running expterminator

To run expterminator, just execute the following command from the shell:
```./expterminator INPUT_FORMULA_PATH [INPUT_PROOF_PATH]```

* `INPUT_FORMULA_PATH` is the path to the QDIMACS file containing a quantified boolean formula.
* `INPUT_PROOF_PATH` is the path to the ∀-Exp+Res proof file.
If this is not specified the tool expects the proof to be input via STDIN.
* expterminator outputs the QRAT proof on STDOUT.

For example, if your formula is in the QDIMACS file 'formula.qdimacs' (located in the directory from which you call expterminator) and your ∀-Exp+Res proof is in the file 'proof.res', then the following command writes its output to STDOUT:

```./expterminator formula.qdimacs proof.res```

## Input Format (∀-Exp+Res Proof)

The input format for the quantified boolean formula is [QDIMACS](http://www.qbflib.org/qdimacs.html).

### Forall Expansion Resolution Proof Trace

This is a simple extension of the sat trace format. The main difference is that
this format needs to introduce the annotations that are used for each expanded
variable. The annotations are always at the start of the file and start with an
"x", followed by the variable indices used in the propositional formula, the
original variables in the QBF and a set of variable assignments in literal
notation, which represent the annotation. The following example illustrates the
meaning of the notation we use:

`x 1 2 0 8 9 0 -2 -4 6 -7 0`

The above line means that 8^{-2 -4 6 -7} is represented by 1 and 9^{-2 -4 6 -7}
is represented by 2 in the propositional formula. The -2 in the annotation is
not to be confused with the 2 in the propositional formula. The variable 2 of
the QBF was assigned the value false and the variable 9 carries that annotation.

In order to make the checking of the proof simpler, we also require that the
origin of a clause is clearly marked. This is done through the antecedents
notation. A clause is either obtained from an instantiation (axiom rule) or
from a binary resolution (res) rule. We will therefore have two cases, as
illustrated by the following examples:

```
1 1 -2 3 0 8 0
2 -1 4 0 11 0
3 -2 3 4 0 1 2 0
```

This means that we introduce the clause with index 1, which has annotated
literals 1, -2, and 3, and which comes from clause 8 in the original QBF.
The clauses of the original QBF are numbered implicitly starting from 1.
We also introduce clause 2 with literals -1, and 4, by instantiating clause 11
of the original QBF. Finally, we introduce clause 3 as the resolution of
clauses 1 and 2 in this proof.

In order to best understand this is to see that the axiom rule can only be
applied to formulas of the QBF which will cause all literals in the generated
clause to be annotated. Rule applications of resolution will alway apply to
clauses generated in this proof.

Also note that the indeces of the new literals and the newly generated clauses
have to start at 1 and continue increasing one at a time.

Here is the definition of the humaly readable FERP format:

```
<trace>       = { <annotattion> } { <clause> }
<annotattion> = "x" <vars> <vars> <literals> EOL
<vars>        = { <pos> } "0"
<literals>    = { <lit> } "0"
<antecedents> = <pos> "0" | <pos> <pos> "0"
<clause>      = <pos> <literals> <antecedents> EOL
<lit>         = <pos> | <neg>
<pos>         =  "1" |  "2" | .... | <max-idx>
<neg>         = "-"<pos>
```
