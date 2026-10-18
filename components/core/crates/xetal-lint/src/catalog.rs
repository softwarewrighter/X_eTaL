//! The lints themselves, as data (see `registry` for the shape).

use crate::registry::{Group, Level, Lint};

pub(crate) const LINTS: &[Lint] = &[
    Lint {
        name: "shadows-builtin",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a parameter or local binding shadows a built-in",
        explain: "What it does: warns when a lambda parameter or a local `:=` has the \
name of a built-in (L7).\n\nWhy: inside its scope the name no longer means the \
built-in, which surprises a reader who knows the built-in.\n\nExample:\n    \
{ r_ev -> r_ev 1 2 3 }\nInstead:\n    { f_ -> f_ 1 2 3 }\n\nAlso printed by `xetal run`.",
    },
    Lint {
        name: "deprecated-private",
        group: Group::Deprecated,
        level: Level::Warn,
        summary: "a library's bare top-level function (write it h:)",
        explain: "What it does: warns on a bare top-level function in a library (PN2).\n\n\
Why: a library's private helpers are written `h:` (`h:j_oin`); bare ones are deprecated.\n\n\
Example (in Lib.xtl):\n    d_ouble := { _r * 2 }\nInstead:\n    h:d_ouble := { _r * 2 }\n\n\
`xetal migrate FILE` rewrites them. Also printed by `xetal run`.",
    },
    Lint {
        name: "unknown-lint",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a `# lint:` directive names no lint, group or level",
        explain: "What it does: warns on a `# lint:` or `# lint-file:` comment whose level \
is not allow, warn or deny, or which names no lint or group.\n\nWhy: a misspelled \
directive would otherwise silence nothing, quietly.\n\nExample:\n    \
# lint: allow shadow-builtin\nInstead:\n    # lint: allow shadows-builtin",
    },
    Lint {
        name: "unused-binding",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a lambda parameter or local binding that is never used",
        explain: "What it does: warns on a lambda's parameter or a local `:=` that nothing \
in its scope uses.\n\nWhy: an unused name is often a slip (the wrong name used below \
it) or code left behind.\n\nExample:\n    { v -> shown := p_rint! v\n    v }\nInstead:\n    \
{ v -> _ := p_rint! v\n    v }\n\nExempt: `_ := ...` (a value \
kept for its effect), the shorthand's `_r` and `_l`, and top-level bindings. A \
parameter a caller requires (an operand ignoring an argument) is silenced with a \
`# lint: allow unused-binding` comment.",
    },
    Lint {
        name: "redundant-lambda",
        group: Group::Complexity,
        level: Level::Warn,
        summary: "a lambda that only passes its arguments to a named function",
        explain: "What it does: warns on a lambda whose body only applies a built-in or a \
named function to its parameters, in order (`{ x -> n_eg x }`, `{ a b -> a + b }`), \
or returns its one parameter (`{ x -> x }`).\n\nWhy: the quoted name says the same \
thing in fewer tokens, and a reader does not have to check what the lambda adds.\n\n\
Example:\n    '{ x -> n_eg x } e_ach v\nInstead:\n    'n_eg e_ach v\n\n\
`{ x -> x }` is `'i_d`. A lambda that is a definition's whole value \
(`u:n_ot := { x -> n_eg x }`) is not reported: it names its parameters for a reader. \
Nor is a lambda passing its arguments to power (`{ a b -> a ^ b }`): power has no \
quoted form, since `'^` is an exponent (D-6).",
    },
    Lint {
        name: "missing-bang",
        group: Group::Style,
        level: Level::Warn,
        summary: "a function that calls an effect but has no `!` in its name",
        explain: "What it does: warns on a top-level function that calls a `!` built-in \
(`p_rint!`, `r_oll!`) or a `!` function of the program or a library, anywhere in \
its body, but whose own name has no `!`.\n\nWhy: lang-choices section 16 ends the \
name of a function with a side effect with `!`, so a reader sees where output, \
time and randomness come from; an unmarked caller hides it.\n\nExample:\n    \
u:s_ay := { v -> p_rint! v }\nInstead:\n    u:s_ay! := { v -> p_rint! v }\n\n\
Not counted: a `!` function passed in as a parameter (the caller's effect), and \
the system functions (`[]S_HOW`), which are named without `!` (QD3).",
    },
    Lint {
        name: "needless-bang",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a `!` function that calls no effect",
        explain: "What it does: warns on a top-level function named with `!` whose body \
could not have an effect: no `!` function, `!` parameter or variable, mutation or \
system function.\n\nWhy: `!` tells a reader to look for an effect; a pure function \
named with one misleads.\n\nExample:\n    u:d_ouble! := { _r * 2 }\nInstead:\n    \
u:d_ouble := { _r * 2 }\n\nOff by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "int-condition",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a guard whose condition is an Int (a count), not a truth",
        explain: "What it does: warns on a guard `cond ? then` whose condition type-checks \
as an Int, such as a count (`t_ally v ? ...`). It runs after type checking, and only \
when the program type-checks.\n\nWhy: a condition takes any truth (T5), so an Int \
type-checks, but at run time it must be 1 or 0 (`error[not-a-bool]` otherwise): a \
count where a truth belongs usually means a comparison was left out.\n\nExample:\n    u:a_ny := { v -> t_ally v ? 1\n    0 }\n\
Instead (a function applies to all on its right):\n    u:a_ny := { v -> 0 < t_ally v ? 1\n    0 }\n\nA comparison is not \
reported: it is a truth that becomes an Int only in arithmetic. On by default; a \
guard meant to test an Int that is always 1 or 0 is silenced with `# lint: allow int-condition`.",
    },
    Lint {
        name: "function-name-case",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a function named UpperCamel (functions are lowerCamel)",
        explain: "What it does: warns on a function whose name starts with a capital and \
is more than one letter.\n\nWhy: the style guide (lang-choices section 16) names \
functions lowerCamel, first letter underlined, and gives a capital start to \
combinators and their variants (`K_`, `C_s`). Decoration, not case, makes a name \
a function, so this is a convention only: off by default.\n\nExample:\n    u:N_extGen := { _r + 1 }\nInstead:\n    \
u:n_extGen := { _r + 1 }\n\nOff by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "mutable-constant",
        group: Group::Style,
        level: Level::Warn,
        summary: "a mutable variable named like a constant (UpperCamel)",
        explain: "What it does: warns on a mutable variable (`!`) whose name starts with a \
capital.\n\nWhy: UpperCamel names a constant (`MaxSize`); a variable that changes \
is lowerCamel.\n\nExample:\n    Count! := 0\nInstead:\n    count! := 0",
    },
    Lint {
        name: "upper-case-name",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a variable named in all capitals",
        explain: "What it does: warns on a variable of two or more letters, all \
capitals.\n\nWhy: the style guide writes a constant UpperCamel (`MaxSize`) and a \
variable lowerCamel (`maxSize`); all capitals hides the word boundaries.\n\n\
Example:\n    MAXSIZE := 9\nInstead:\n    MaxSize := 9\n\nOff by default: \
`xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "macro-capture",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a macro binds a name the call's own text reads (a capture)",
        explain: "What it does: warns where the text of a macro call reads a name that \
the macro's expansion binds, when the macro wrote that name itself, did not get it \
from `[]G_ENSYM` and does not declare it in its `## binds:` (MC30).\n\nWhy: a name a \
macro writes stays as written, so the caller's `x` silently becomes the macro's `x`. \
A capture meant for the caller (an anaphor) is declared; any other is a slip.\n\n\
Example (the macro writes `x := 3` around its argument):\n    x := 10\n    \
@ x:b_ad< \"x + 1\"\nInstead (in the macro library):\n    \
m:b_ad< := { @ e ->\n      x := []G_ENSYM \"x\"\n      \"{ @ -> \" c_at x c_at \" := 3; \" c_at e c_at \" } @\" }\n\n\
Or, when the capture is meant, `## binds: x` in the macro's doc block. Reported at \
each read in the call's text.",
    },
    Lint {
        name: "unused-anaphor",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a call's text never reads a name the macro binds for it",
        explain: "What it does: warns on a macro call whose text never reads a name the \
macro declares it binds for that text (`## binds: it`, an anaphor).\n\nWhy: the \
macro offers the name for the text to use; not using it may mean another macro was \
meant. The binding is the macro's, so the caller cannot write `_ :=` for it: it is \
reported here, at the call, not as `unused-binding`.\n\nExample:\n    \
\"3 + 4\" x:w_ith< \"5\"\nInstead:\n    \"3 + 4\" x:w_ith< \"it * it\"\n\n\
Off by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "macro-internals",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a finding in code a macro wrote, not the call's text",
        explain: "What it does: reports a finding of another lint in code a macro's \
expansion wrote (not text copied from the call), naming the macro and the lint.\n\n\
Why: the caller cannot change code the macro writes, so such findings are not \
reported as the caller's; they belong to the macro's author.\n\nExample (the \
macro writes `tmp := 1; ` before its argument, and never reads `tmp`):\n    \
@ x:w_aste< \"1\"\nInstead (in the macro library): write `_ := 1; `, or drop it.\n\n\
Off by default: `xetal lint -W macro-internals` shows them.",
        name: "shape-mismatch",
        group: Group::Correctness,
        level: Level::Warn,
        summary: "shapes that surely differ where they must agree (fails at run time)",
        explain: "What it does: carries shapes through the program from literals, \
`r_eshape` and `r_equireShape` checks (type-check-macros approach 8), by the shape rules \
the catalog lists beside each built-in's type, and warns where it proves a failure: \
an item-by-item function on two arrays of different shapes, an inner product whose \
paired axes differ, or an `r_equireShape` check that cannot pass. Extents may be names \
(`N c_at 3 r_equireShape X`), so a macro's checks are facts the lint reads without \
knowing the macro's notation.\n\nWhy: each of these is `error[shape-mismatch]` at run \
time; found before running, it is found at the place that is wrong.\n\nExample:\n    \
X := 2 3 r_eshape 0\n    X + 3 2 r_eshape 1\nInstead:\n    X + 2 3 r_eshape 1\n\nIt \
speaks only where it proves a mismatch: where a size depends on data (a parameter, a \
file read), it says nothing and the run-time check stays the backstop.",
    },
    Lint {
        name: "shape-check-proven",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "an `r_equireShape` check the shape lint proves always passes",
        explain: "What it does: reports an `r_equireShape` check whose argument's shape the \
shape lint already knows to be the checked one, extent by extent (the same numbers, \
or the same named extents).\n\nWhy: the check costs a test at run time and says \
nothing a reader does not already know there; a macro may emit such checks freely, \
and this shows the ones that can go.\n\nExample:\n    X := 2 3 r_eshape 0\n    \
2 3 r_equireShape X\nInstead:\n    X\n\nOff by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "needless-raw-hashes",
        group: Group::Style,
        level: Level::Warn,
        summary: "a raw string with more `#` than it needs",
        explain: "What it does: warns on a raw string (ST4) written with more `#` than \
it needs to end where it does: the fewest is none when it holds no `\"`, else one \
more than the most `#` after a `\"` in it.\n\nWhy: each `#` tells a reader to look \
for a `\"#` inside; extra ones send them looking for nothing.\n\nExample:\n    \
s := r##\"a \"quoted\" word\"##\nInstead:\n    s := r#\"a \"quoted\" word\"#\n\n\
`--fix` rewrites the delimiters; the value is unchanged.",
    },
    Lint {
        name: "needless-raw-string",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a raw string with no backslash, quote or newline (plain is the same)",
        explain: "What it does: warns on a raw string `r\"...\"` or `r#\"...\"#` whose \
text has no backslash, no `\"` and no newline.\n\nWhy: a plain string spells the \
same value, and a reader need not recall the raw rules.\n\nExample:\n    \
s := r\"plain text\"\nInstead:\n    s := \"plain text\"\n\n`--fix` drops the `r` \
and any `#`. Off by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "escapes-could-be-raw",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a string with several `\\\"` or `\\\\` escapes (clearer raw)",
        explain: "What it does: warns on a plain string with two or more of the escapes \
`\\\"` and `\\\\` and no `\\n` or `\\t`.\n\nWhy: a raw string (ST4) writes the text as \
it is, which is easier to read, above all for code passed to a macro.\n\n\
Example:\n    c := \"x := \\\"3 + 4\\\"\"\nInstead:\n    c := r#\"x := \"3 + 4\"\"#\n\n\
`--fix` writes it raw with the fewest `#` that work; the value (what a macro \
receives, D156) is unchanged. Off by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "string-holds-directive",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a line inside a string that reads as a `# lint:` directive",
        explain: "What it does: warns on a line of a string, of any kind, whose text \
starts with `# lint:` or `# lint-file:`.\n\nWhy: inside a string it is text and sets \
no level; the author probably meant a comment.\n\nExample:\n    \
s := r\"one\n    # lint: allow unused-binding\n    two\"\nInstead:\n    \
# lint: allow unused-binding\n    s := r\"one\n    two\"\n\nNo fix: only the author \
knows whether the text belongs in the string.",
    },
];
