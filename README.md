# Read Me Or Don't, I Don't Care

An implementation of [this paper](https://benjaminkiesl.github.io/publications/qrat_simulates_forall_exp_res_kiesl_seidl.pdf)

## Assumption

The tool is written such that it assumes that the inputs are correct.
It does not guarantee that it works properly if the inputs are broken.

## Usage

Run
```expterminator qbf_file```
and feed in the proof via `stdin` or
```expterminator qbf_file all_exp_res_proof```

## Format (QBF)

[QDIMACS](http://www.qbflib.org/qdimacs.html)

## Format (Forall-Exp+Res* Proof)

Forall Expansion Resolution Proof Trace

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

## Other

Some of the comments or comments refer to variables from the paper.