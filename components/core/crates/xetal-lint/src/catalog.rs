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
        name: "gensym-unused",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a fresh name from []G_ENSYM that the macro never uses",
        explain: "What it does: warns on a `[]G_ENSYM` call in a macro library whose \
result is thrown away, or bound to a name nothing reads.\n\nWhy: a macro asks for \
a fresh name to write into the text it gives (MC30); one it never writes there \
means the text uses a name of its own instead, which a caller's name can capture.\n\n\
Example (in M.xtlm):\n    m:t_wice< := { @ e ->\n      t := []G_ENSYM \"t\"\n      \
\"{ @ -> t := 2; t * (\" c_at e c_at \") } @\" }\nInstead:\n    m:t_wice< := { @ e ->\n      \
t := []G_ENSYM \"t\"\n      \"{ @ -> \" c_at t c_at \" := 2; \" c_at t c_at \" * (\" c_at e c_at \") } @\" }\n\n\
Reported instead of `unused-binding` at the same name.",
    },
    Lint {
        name: "gensym-not-name",
        group: Group::Correctness,
        level: Level::Warn,
        summary: "[]G_ENSYM given a literal that is not a plain name",
        explain: "What it does: warns on `[]G_ENSYM \"...\"` whose literal is not a plain \
name: a letter, then letters and digits, with at most one underline, second (`t`, \
`s_um`).\n\nWhy: the call fails with error[bad-gensym] whenever the macro runs, \
but only a call shows it; the library itself loads.\n\nExample:\n    \
t := []G_ENSYM \"the total\"\nInstead:\n    t := []G_ENSYM \"total\"\n\n\
A name computed at expansion is not checked.",
    },
    Lint {
        name: "reject-bad-place",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a []R_EJECT code whose place is not left, right or call",
        explain: "What it does: warns on a literal `\"code place\" []R_EJECT message` whose \
place is not `left`, `right` or `call`.\n\nWhy: an unknown place is ignored, so the \
error lands on the macro's name instead of the argument meant.\n\nExample:\n    \
\"bad-macro-argument rigth\" []R_EJECT \"an empty argument\"\nInstead:\n    \
\"bad-macro-argument right\" []R_EJECT \"an empty argument\"",
    },
    Lint {
        name: "reject-undocumented",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a macro rejects its call with a code its doc block never mentions",
        explain: "What it does: warns on a literal code a macro rejects its call with \
(`\"code place\" []R_EJECT message`) that the macro's `##` block does not name, as a \
word in its prose or its examples.\n\nWhy: a macro's users meet the code as \
`error[code]`; one nobody documented sends them to the library's source.\n\n\
Example:\n    ## Its argument.\n    m:i_d< := { @ e ->\n      0 = t_ally e ? \
\"empty-argument right\" []R_EJECT \"an empty argument\"\n      e }\nInstead:\n    \
## Its argument, which may not be empty (empty-argument).\n\n\
Each code once a macro; a code in a helper (`h:`), or one computed, is not checked. \
Off by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "reject-unplaced",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a []R_EJECT code with no place, so the error lands on the macro's name",
        explain: "What it does: warns on a literal `\"code\" []R_EJECT message` with no \
place after the code.\n\nWhy: with no place the error is reported at the macro's \
name; naming the argument at fault (`left`, `right`) points the user at what to \
change, and `call` says the whole call was meant.\n\nExample:\n    \
\"bad-macro-argument\" []R_EJECT \"an empty argument\"\nInstead:\n    \
\"bad-macro-argument right\" []R_EJECT \"an empty argument\"\n\n\
Off by default: `xetal lint -W pedantic` turns it on.",
    },
    Lint {
        name: "missing-example",
        group: Group::Pedantic,
        level: Level::Allow,
        summary: "a macro with no `## >>` example in its doc block",
        explain: "What it does: warns on a macro (`m:n_ame< := ...`) whose doc block has no \
`## >>` example, or that has no doc block.\n\nWhy: a macro is read by the text it \
gives; an example shows a call and its result, and `xetal doc --test` keeps it true.\n\n\
Example:\n    ## Its argument.\n    m:i_d< := { @ e -> e }\nInstead:\n    \
## Its argument.\n    ## >> \"x:\" u_se< \"M\"\n    ## >> @ x:i_d< \"3\"\n    ## 3\n    \
m:i_d< := { @ e -> e }\n\nOff by default: `xetal lint -W pedantic` turns it on.",
    },
];
