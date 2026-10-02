# ADVANCEDEX: IBM's advanced examples for APL\360

IBM shipped a workspace, ADVANCEDEX (saved 07/20/68), so that APL\360
users could load, run, change and trace real programs. Its functions
are printed in Appendix B, Advanced Examples, of the APL\360 User's
Manual: the August 1968 edition and the March 1970 edition
(GH20-0683-1), both at softwarepreservation.org. This page is the
inventory: one row per function, what it does, the APL\360 features
it uses, and the X_eTaL it maps to or the feature X_eTaL still needs.
Each function's text, as printed, is in `docs/apl/advancedex/NAME.md`.

Status: the 32 names are listed; the transcriptions and the columns
below are still to be filled in from the manual (the sandbox that
started this page could not read Appendix B; see
`lanes/classics/HANDOFF.md`).

## How the transcriptions are written

- One file per function, `docs/apl/advancedex/NAME.md`, holding the
  function exactly as printed, line numbers in brackets, in a fenced
  block, with the edition and page it came from.
- APL glyphs are written as HTML character references (`&#9035;` for
  the grade-up, `&#8592;` for the assignment arrow, and so on), so the
  markdown stays ASCII; a rendered page shows the glyphs.
- Where the two editions differ, both are given, 1968 first.

## The functions

| Function | What it does (from the manual) | Edition, page | APL\360 features used | In X_eTaL (or the feature needed) |
| -------- | ------------------------------ | ------------- | ---------------------- | --------------------------------- |
| [AH](advancedex/AH.md) | to transcribe | | | |
| [ASSOC](advancedex/ASSOC.md) | to transcribe | | | |
| [BIN](advancedex/BIN.md) | to transcribe | | | |
| [COMB](advancedex/COMB.md) | to transcribe | | | |
| [DTH](advancedex/DTH.md) | to transcribe | | | |
| [ENTER](advancedex/ENTER.md) | to transcribe | | | |
| [F](advancedex/F.md) | to transcribe | | | |
| [FC](advancedex/FC.md) | to transcribe | | | |
| [GC](advancedex/GC.md) | to transcribe | | | |
| [GCD](advancedex/GCD.md) | to transcribe | | | |
| [GCV](advancedex/GCV.md) | to transcribe | | | |
| [HILB](advancedex/HILB.md) | to transcribe | | | |
| [HTD](advancedex/HTD.md) | to transcribe | | | |
| [IN](advancedex/IN.md) | to transcribe | | | |
| [INV](advancedex/INV.md) | to transcribe | | | |
| [INVP](advancedex/INVP.md) | to transcribe | | | |
| [IN1](advancedex/IN1.md) | to transcribe | | | |
| [LFC](advancedex/LFC.md) | to transcribe | | | |
| [LOOKUP](advancedex/LOOKUP.md) | to transcribe | | | |
| [PACK](advancedex/PACK.md) | to transcribe | | | |
| [PALL](advancedex/PALL.md) | to transcribe | | | |
| [PER](advancedex/PER.md) | to transcribe | | | |
| [PERM](advancedex/PERM.md) | to transcribe | | | |
| [PO](advancedex/PO.md) | to transcribe | | | |
| [POL](advancedex/POL.md) | to transcribe | | | |
| [POLY](advancedex/POLY.md) | to transcribe | | | |
| [POLYB](advancedex/POLYB.md) | to transcribe | | | |
| [RESET](advancedex/RESET.md) | to transcribe | | | |
| [TIME](advancedex/TIME.md) | to transcribe | | | |
| [TRUTH](advancedex/TRUTH.md) | to transcribe | | | |
| [UNPACK](advancedex/UNPACK.md) | to transcribe | | | |
| [ZERO](advancedex/ZERO.md) | to transcribe | | | |

## Features X_eTaL is known to need

- Matrix inverse and division (APL's domino), for INV and INVP; to be
  decided with the user when the ports reach them.
- Execute (reading text as a program), if a function builds and runs
  an expression; reserved (lang-choices QD3).
- Nested arrays are now in the language (B14, B16), for any function
  that keeps items of different lengths.

The columns above will say which functions need which.
