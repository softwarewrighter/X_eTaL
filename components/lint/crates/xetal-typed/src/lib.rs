//! The typed lints: passes that need the types inference gives, so they
//! run only when the program type-checks (docs/lint.md). Apart from
//! xetal-lint, which must not depend on the checker (the checker
//! depends on Core); their catalog entries are there all the same.

mod guard;

pub use guard::typed;
