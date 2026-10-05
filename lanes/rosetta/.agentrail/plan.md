rosetta lane: Saga 34 in docs/plan.md, the idioms compared on an impossible rotating stone (design: docs/rosetta.md; the conversation: docs/research5.txt, archival). The demo is an X_eTaL program, Rust its shell; the data one data.toml read as aligned arrays. One PR per step from the latest origin/main; TDD; the gate before every commit; decisions with the user at step 1 and recorded in lang-choices before anything is built; the cube (step 4) is the review point before the stone.

Steps
1. rosetta-decisions: []E_VENT, []T_ABLE, the TOML schema, the first lists, with the user.
2. geometry-lib: lib/Geometry3D.xtl.
3. svg-lib: lib/Svg.xtl.
4. cube: the rotating cube from the two libraries; the review point.
5. events: []E_VENT, the CLI's scripted queue, the browser bridge.
6. table: []T_ABLE, the TOML loader, data.toml, just rosetta-check.
7. split-face: the three carousels, one static comparison.
8. axes: the Axis abstraction, indexes as truth, transition tests.
9. pointer: drags, clicks, inertia, snapping, the short-way turn.
10. attract: the nested traversal with dwell and overlap.
11. faces: coloring, layout, the missing-implementation face.
12. web-host: the Yew page, controls, URL, keyboard, reduced motion.
13. idioms-doc: docs/idioms.md generated from data.toml.
14. profile: frame time against the budget.
15. rosetta-release: literate document, screenshots, links, dogfooding, CHANGES, pages; lane archived.
