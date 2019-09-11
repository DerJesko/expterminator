use crate::qbf::Clause;

struct AllExpResProof {
    rules: AllExpResRule,
}

enum AllExpResRule {
    Axiom(Clause),      // The clause which is created by the axiom rule
    Resolution(Clause), // The clause which is created by resolution
}
