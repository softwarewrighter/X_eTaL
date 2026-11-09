# reg-rs goldens: an audit and the improvements it asks for

An audit of the CLI goldens in `reg/` (271 baselines at 3376a1be,
2026-11-02), asked for by the user: do they test something, or are some
contrived, going through the motions, or of little value? This file
lists what was found and what to change. Nothing here has been changed
yet; each item names its fix.

## How it was audited

- Every `reg/*.rgt` was read with its `.out` and `.err`, and the
  fixture or program it runs where the verdict depended on it.
- Each golden's `-e` source and stage was matched against the
  `== SOURCE` sections of `spec/**/*.case`: the spec harness
  (`components/cli/crates/xetal-cli/tests/spec.rs`) already runs
  `xetal <stage> -e SRC` and checks the result, so a golden repeating a
  case adds nothing.
- Mechanical checks over all baselines: empty or tiny output, commands
  that hide output or exit codes (`grep -c`, `head`, `cut`,
  `>/dev/null`, `;`), FAIL lines, `error[` with exit 0, identical
  commands and identical outputs.
- The claims below marked (verified) were rerun or reread by hand; the
  rest come from the read review and quote the files.
- The sibling repositories (X_eTaL-libraries, X_eTaL-ML) were also
  mutation-tested; X_eTaL's own Rust was not (each mutant is a rebuild),
  so here the read review stands alone.

## Verdict

Most goldens test real CLI behavior: about 208 of 271 are solid. About
63 have a problem; two pin wrong output. Nothing tautological was
found. The largest groups are redundancy (23), masked commands (15) and
brittleness (15).

Solid examples, kept as models: `cli-pipe-closed` (SIGPIPE exit 141
and an empty stderr across four variants), `lint-json`, `lint-fix`,
`lint-fix-strings` (Core the same before and after the fixes),
`edit-screen-run`, `repl-session`, `lib-runtime-error-caller`,
`eval-keeps-output`, `signal-stops`, `read-to-end`, `roll-range`
(pinned by a property through `preprocess`), `roll-seed-repeats` with
`roll-unseeded-differs`, `doc-test-failures`, the usage and IO errors
(`cli-no-input`, `cli-unreadable`, `cli-no-subcommand`), the lint level
and group goldens, `run-rosetta-transitions` (43 hand-written checks)
and the classics demos with self-checks (closure, quicksort, roman,
magic).

## 1. Wrong output pinned (fix the code, then rebase)

| Golden | What is wrong | Fix |
| ------ | ------------- | --- |
| `run-table-errors` (verified) | for `"nope.toml" []L_IST "rows"` the pinned message is `error[io]: []T_ABLE: nope.toml: No such file...`: it names the wrong built-in | name the built-in called (`[]L_IST`), then rebase |
| `macros-notation-drawer-missing` (verified) | the message is right (`e_q< draws by t_ex<, which this library does not define`) but its span is line 2, `## takes: notation text`, of `spec/macros/data/DrawerMissing.xtlm`, not line 3, `## draws: t_ex<` | span the `draws:` line |

## 2. Goldens that run a binary the gate does not build

11 goldens run `./target/release/xetal`: `run-classics-mandelbrot`,
`run-classics-turtle`, `run-rosetta-angled`, `-attract`, `-cube`,
`-drag`, `-keys`, `-stone`, `-tumble`, `run-tttml`, `tttml-play`. Since
2026-10-20 the sample gate builds only debug (`scripts/gate.sh` lines
104-113), so by day these run whatever release binary was last built,
and `scripts/reg.sh` (lines 14-15) still says "Debug only: no golden
runs the release binary" (verified). The `just-*` goldens call
`scripts/quick-build.sh`, which rebuilds `target/release` in the middle
of the parallel run when it is stale.

Fix: run the 11 on `target/debug/xetal` (with a `timeout` for TTTML),
or have `reg.sh` build release whenever a golden names it; correct the
comment either way.

## 3. Masked: a regression could still pass

| Golden | What is hidden | Fix |
| ------ | -------------- | --- |
| `cli-help` | `grep -c 'AI CODING AGENT'` gives `0` and `1`: one marker line, nothing else of `-h` or `--help` is pinned | pin `-h` whole, and `--help \| sed -n '/AI CODING/,$p'` |
| `cli-too-deep` | `2>&1 \| cut -c1-17` keeps only `error[too-deep]: `; no stage's exit code is seen, the loop exits 0 | drop `cut`, add `; echo "exit $?"` per stage |
| `type-errors-boxed`, `-condition`, `-int-and-float`, `-quote`, `-rebind`, `-string`, `-waiting` | `broken; fixed`: exit 0 comes from the fixed run, the broken run's failure is not checked | `broken \|\| echo "exit $?"; fixed` |
| `lint-fix-dirty`, `lint-fix-rename`, `lint-fix-unused` | `lint --fix ...; cmp` hides the lint's exit status | echo `$?` after the lint |
| `doc-header-and-stray` | `\| head -2` would hide a doc block wrongly attached after the second entry | drop `head` |
| `cli-early-errors` | the first command's exit code is hidden by `;` | echo `$?` |
| `names-helpers-run` | empty stdout, exit 0: a file treated as a library with no exports also prints nothing | check how it was loaded (`xetal doc --json ... \| grep '"kind"'`, program) |
| `lint-clean` | `lint -W all -e 'x := 1 2 3'`: a trivial input | lint a real clean program, e.g. `demos/life.xtl -W all` |
| `run-classics-mandelbrot` | stdout empty; only the three `drawn ...svg` names are compared, never the pictures | pin one SVG, or print an iteration count |

Cleared on inspection: `version` (`head -4` is deliberate; build info
is in `cli-version-build`) and the `doc-site-*` goldens (their greps
extract specific anchors and markup).

## 4. Redundant: delete or merge

Exact duplicates of spec cases (same stage, source and output), 17:
`lex-life`, `parse-life`, `core-life`, `fmt-life`, `render-life`
(`spec/lex/life`, `spec/syntax/life`, `spec/render/life`);
`lex-square` (`spec/lex/square`); `parse-sub`, `core-sub`
(`spec/syntax/sub`); `eval-add` (`spec/eval/add`);
`eval-shape-mismatch` (`spec/eval/reject-shape-mismatch`);
`eval-int-plus-float` (`spec/errors/explain-literal`);
`lex-ambiguous-minus` (`spec/lex/reject-ambiguous-minus`);
`parse-rejects-section` (`spec/ambiguity/symbol-one-argument`);
`macros-error-in-argument` (`spec/macros/argument-error-located`);
`macros-user-expand`, `macros-user-run` (`spec/macros/user-macros`);
`eval-unit-reject` (nearly `spec/eval/reject-not-unit`).

Duplicates of each other, 6:

- `just-run-flags` has the same output as `run-echo`, and
  `notebook-factorial` is `run-echo` without colors: keep `run-echo`,
  point `just-run-flags` at a flag it alone checks
  (`just run --untyped demos/fixed-point.xtl`), delete
  `notebook-factorial`.
- `hello-private-name`, `lib-not-exported` and `names-library-hidden`
  are all an importer naming an `h:` helper: keep one.
- `run-echo-tour` (350 lines) is `run-tour` (150) with decoration, so
  every tour edit rebases both: delete it.
- `roll-dice` is a smaller `roll-range`: delete it.

Kept although their outputs match: `lib-error-located` and
`lint-load-error`, `docs-life-line` and `render-life-roundtrip` (each
pair tests two different behaviors).

## 5. Brittle: rebased without a behavior change

- `just-list` pins all 53 lines of `just --list`, recipe descriptions
  included; that is not xetal behavior. Delete it, or check that a few
  recipes exist.
- `core-trains` pins the generated names `%1 %2 %3`: move it to a spec
  `== CORE` section or normalize the names.
- `fmt-factorial` writes the fixed `/tmp/xetal-fmt-factorial.xtl`,
  against the rule that a golden writes under its own path in `work/`.
- `cli-version-build` reads `sed -n '6,9p'`: by line position.
- `run-rosetta-stone`, `-angled`, `-tumble` and `-cube` pin whole SVGs
  with float coordinates: deliberate, but any geometry change rebases
  them all. Pin a few coordinates, or keep one.

No dates, hosts, commit hashes or absolute paths are pinned (grepped).

## 6. The description claims more than the command shows

| Golden | Claim | Reality / fix |
| ------ | ----- | ------------- |
| `learn-xetal` | "each prints what its comment says" | nothing compares the `# value` comments to the output: check them with a script |
| `long-prefixes-color` | an alias drawn raised where every letter has a superscript | no `combinators:` name is used: add `1 combinators:K_ 2` |
| `lint-allow-lint-over-group` | "whatever the order" | one order only: add the reverse |
| `names-app-interface` | u: names and bare variables | `reg/fixtures/names/app.xtl` has no bare variable |
| `system-library-types`, `macros-library-types` | "each (String, String) -> String" | the pins are `Char -> Char -> Char` and others: correct the desc |
| `run-tour` | "every implemented feature, checked" | a pinned output, not checks |
| `run-events` | events taken apart by `[]E_KEY` | `key Up` prints `key  `: the key name never shows |
| `lint-load-error` | "a program that does not load" | `Oops.xtl` is a library |
| `run-classics-mandelbrot` | drawn as images | the images are not compared |

## 7. Cosmetic

- The FAIL lines in `macros-demo-run`, `macros-user-run`,
  `macros-user-expand` and `run-box-macro` are intended: they come from
  the deliberately false `c"5 = 2 * 2" m:c_heck< "doubling"`
  (`lib/Macros.xtlm`). They make a search for FAIL noisy; consider
  wording the demo's check output `not ok:`.
- `macros-run` and `macros-expand` define `u:max_s` as a sum
  (`"sum max" e_ach<`): a template whose name does not mislead.

## Suggested order

1. The two wrong outputs (section 1) and the release-binary goldens
   (section 2): they are bugs or can fail for reasons unrelated to the
   change under test.
2. Unmask section 3: small command edits, then rebase on purpose.
3. Delete the 23 redundant goldens (section 4).
4. Sections 5 and 6, as the goldens are next touched.
