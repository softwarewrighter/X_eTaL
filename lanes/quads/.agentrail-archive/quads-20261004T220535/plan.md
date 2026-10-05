quads lane: Saga 13's system values and the clock, for the sibling asks X4 (X_eTaL-libraries: []U_CS, []A, []D, []TS) and E4 (X_eTaL-extensions: a clock, []TS and []D_L). A lane (one PR per step, each from the latest origin/main) whose saga lives in lanes/quads/.agentrail; run agentrail with --saga lanes/quads. Started 2026-10-04 at the user's go-ahead.

Decided with the user (QD2, QD3; 2026-10-04 for QD7): []U_CS takes characters to codes (Char -> Int) and []U_CHAR codes to characters (Int -> Char), one static type per name; []TS is local time. []R_EAD exists; []V_ALUE and typed execute are left for their own step with the user.

Steps
1. quad-values: the read-only system values []A, []D, []AV, []IO (a quad name without an underline is a value, lowered to a niladic built-in) and []U_CS, []U_CHAR; spec cases, rejections, the reference, asks X4 partly landed.
2. quad-clock: []TS (local time stamp) and []D_L (delay) through the host store (the command line's clock and sleep; the browser's Date and a host without a clock answering with an error, never a panic); asks X4 and E4.