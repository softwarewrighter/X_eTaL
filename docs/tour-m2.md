# A tour of M2

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M0 and M1](tour-m0-m1.md); next: [A tour of M3](tour-m3.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M2 -- static types.** Types are inferred, never written: `xetal type`
prints one line per top-level item, and `xetal eval` and `xetal run`
check the program before anything runs. A niladic function takes Unit,
written `@`:

```
# demos/unit.xtl
u:a_nswer := { @ -> 42 }
u:a_nswer @
```

```bash
./target/release/xetal type demos/unit.xtl       # u:a_nswer : Num a => Unit -> a, then Int
./target/release/xetal run demos/unit.xtl        # 42
./target/release/xetal type demos/factorial.xtl  # u:f_act : Num a => a -> a, then Int
./target/release/xetal eval -e 'u:a_nswer := { @ -> 42 }; u:a_nswer 42'
```

The last command is refused before evaluation:
`error[type-mismatch]: expected Unit, found a number at 26..38`.

Numbers are typed as in Haskell. A literal fits any number type, so
`3 + 2.5` is fine, but a top-level variable has one type: after
`n := 3`, `n` is an Int and `n + 2.5` is a type error. Convert with
`f_loat`:

```bash
./target/release/xetal eval -e 'n := 3; (f_loat n) + 2.5'   # 5.5
```

Comparisons give Bool, which counts as 1 or 0 in arithmetic, and `/`
always gives a Float. The Y combinator's self-applied argument has no
finite type; `--untyped` skips the checker (see above).
