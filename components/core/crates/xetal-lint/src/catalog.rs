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
parameter a caller requires but the lambda ignores (an operand dropping an \
argument, a combinator such as K) is written `_u`: `{ x _u -> x }`.",
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
    },
    Lint {
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
    Lint {
        name: "takes-mismatch",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a macro's `## takes:` line that its parameters contradict",
        explain: "What it does: warns on a macro whose `## takes:` line (MC31) says a side \
takes text where its parameter is `@`, or `@` where its parameter is named.\n\nWhy: \
the runner refuses every call of another shape before the macro runs, so a side \
declared text but written `@` never sees the text it is given, and a named side \
declared `@` names nothing.\n\nExample:\n    ## takes: text text\n    \
m:o_ne< := { @ e -> e }\nInstead:\n    ## takes: @ text\n    m:o_ne< := { @ e -> e }",
    },
    Lint {
        name: "ignored-argument",
        group: Group::Correctness,
        level: Level::Warn,
        summary: "a copying macro that never reads one of its sides",
        explain: "What it does: warns on a macro that copies its text (`## uses: copied`, \
the default, MC31) but never reads one of its named parameters.\n\nWhy: a copying \
macro must write every name and number of its arguments into its expansion; one it \
never reads fails every call that gives text there with error[unused-argument].\n\n\
Example:\n    ## takes: text text\n    m:l_eft< := { a b -> a }\nInstead:\n    \
## takes: text @\n    m:l_eft< := { a @ -> a }\n\nA macro that reads a notation \
says `## uses: parsed` and is not checked. Reported instead of `unused-binding` at \
the same parameter.",
    },
    Lint {
        name: "string-kind",
        group: Group::Style,
        level: Level::Warn,
        summary: "a plain string where a macro takes code, or a code string where it takes text",
        explain: "What it does: warns where a macro's contract (`## takes:`, MC31) says a \
side takes code and the call gives a plain or raw string, or says it takes text and \
the call gives a code string `c\"...\"` (ST5).\n\nWhy: a code string is drawn as code \
and checked with the file (lexed, parsed, its inside type-checked and linted); text \
that is X_eTaL should say so, and text that is not (a label, a notation) should not.\n\n\
Example:\n    \"2 > 0\" i_f< \"1; -1\"\nInstead:\n    c\"2 > 0\" i_f< c\"1; -1\"\n\n\
The fix adds or drops the `c` and keeps the value: `\"...\"` and `c\"...\"` share their \
escapes, `r#\"...\"#` and `c#\"...\"#` are both verbatim.",
    },
    Lint {
        name: "code-check",
        group: Group::Correctness,
        level: Level::Deny,
        summary: "the inside of a code string a macro takes as code does not check on its own",
        explain: "What it does: checks the inside of a code string passed where its macro \
takes code (`## takes: @ code`) before expansion, as the body of a lambda over the \
names it uses (the macro's `## binds:` and every other plain name, each of a free \
type), with the program's functions in scope; an unknown function, a wrong number \
of arguments or a type that cannot fit is reported inside the string.\n\nWhy: a \
macro that answers for its text itself (`## uses: parsed`) leaves no expansion that \
shows the mistake where it was written.\n\nExample:\n    @ x:s_tage< c\"n_egg it\"\n\
Instead:\n    @ x:s_tage< c\"n_eg it\"\n\nA partial check: what holds for every value \
of the free names. Code with `_l`/`_r` beside named parameters or with a mutable \
name is not checked here; the expansion is checked as always.",
    },
    Lint {
        name: "binds-unwritten",
        group: Group::Suspicious,
        level: Level::Warn,
        summary: "a `## binds:` name that no text the macro writes contains",
        explain: "What it does: warns on a macro whose `## binds:` line (MC30) names an \
anaphor that no string literal in its body has as a word.\n\nWhy: the line stops a \
call's text from binding that name (shadows-anaphor); if the macro never binds it, \
the line refuses good calls for nothing, and is likely left from an older body.\n\n\
Example:\n    ## binds: it\n    m:w_ith< := { e body -> \"{ @ -> x := (\" c_at e c_at \
\"); \" c_at body c_at \" } @\" }\nInstead:\n    ## binds: it\n    m:w_ith< := { e body \
-> \"{ @ -> it := (\" c_at e c_at \"); \" c_at body c_at \" } @\" }\n\nA name written by \
a helper the macro calls is not seen: allow it there.",
    },
];
