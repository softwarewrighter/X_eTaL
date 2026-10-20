draws lane: main's step 154 (notation-draws, MC32) worked on a branch so
the main saga's cursor (113) is untouched; the integrator closes 154
when this merges. Decided with the user 2026-10-19: contracts only (no
new string kind): a side declared `notation`, `## draws: m_acro<`
naming a macro of the same library; the program phase (which loads the
libraries) runs the draws: macro per call and records the drawing on
the call's site; views take the overlay: HTML views (doc site, live
demo) draw the notation strings as one math box (KaTeX) after the
macro name and the binding, which stay as code; the terminal shows the
typed text styled as notation. One PR; TDD; just gate; register; CHANGES.

Steps
1. draws-notation: the whole of the above.
