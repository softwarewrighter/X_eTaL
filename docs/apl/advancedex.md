# ADVANCEDEX: IBM's advanced examples for APL\360

IBM shipped a workspace, ADVANCEDEX (saved 07/20/68), so that APL\360
users could load, run, change and trace real programs. Its functions
are printed in Appendix B, Advanced Examples, of the APL\360 User's
Manual: the August 1968 edition and the March 1970 edition
(GH20-0683-1), both at softwarepreservation.org. This page is the
inventory: one row per function, what it does, the APL\360 features
it uses, and the X_eTaL it maps to or the feature X_eTaL still needs.
Each function's text, as printed, is in `docs/apl/advancedex/NAME.md`.

Status: all 32 functions are transcribed from the scanned pages of
both editions (Appendix B, pages B.2 to B.12; the March 1970 text is
given, and the August 1968 edition prints every function the same).
Each file also holds the example session printed with it.

## How the transcriptions are written

- One file per function, `docs/apl/advancedex/NAME.md`, holding the
  function exactly as printed, line numbers in brackets, in a fenced
  block, with the edition and page it came from.
- APL glyphs are written as HTML character references (`&#9035;` for
  the grade-up, `&#8592;` for the assignment arrow, and so on), so the
  markdown stays ASCII; a rendered page shows the glyphs.
- Where the two editions differ, both are given. Only one example
  differs: the 1970 edition omits a closing quote in IN's session.
- APL\360's stile is written `&#8739;` (residue and magnitude); the
  factorial and binomial, printed as an overstruck quote and dot, are
  written `!`; TIME's I-beam is `&#9014;`.

## The functions

| Function | What it does (from the manual) | Edition, page | APL\360 features used | In X_eTaL (or the feature needed) |
| -------- | ------------------------------ | ------------- | ---------------------- | --------------------------------- |
| [AH](advancedex/AH.md) | Adds two hexadecimal numbers | 1970 B.8; 1968 B.8, same | calls DTH and HTD | Maps directly |
| [ASSOC](advancedex/ASSOC.md) | Whether a group's multiplication table is associative (1 or 0) | 1970 B.12; 1968 B.12, same | and-reduction, ravel, indexing a matrix by a matrix on one axis, comparison | And-reduction of the ravel; needs selecting rows (or columns) by an array of indices |
| [BIN](advancedex/BIN.md) | The matrix of binomial coefficients up to order N | 1970 B.5; 1968 B.5, same | outer product (binomial), transpose, floor, iota | An outer product of a binomial, which X_eTaL lacks (APL's dyadic !; computed from factorials meanwhile), and transpose (not yet in X_eTaL) |
| [COMB](advancedex/COMB.md) | All pairs from the first N counts, as a matrix, recursively | 1970 B.7; 1968 B.7, same | recursion, branch, reshape, ravel, catenate, outer product (max) | Recursion with a guard in place of the branch; the rest maps directly |
| [DTH](advancedex/DTH.md) | Decimal to 8-digit hexadecimal (two's complement) | 1970 B.8; 1968 B.8, same | encode, indexing, ravel | e_ncode and indexing |
| [ENTER](advancedex/ENTER.md) | Reads names and their data into global lists until an empty line | 1970 B.3; 1968 B.3, same | quote-quad input, branch loop, globals, catenate | []R_EAD, recursion for the loop, ! names for the growing lists |
| [F](advancedex/F.md) | The sample polynomial for ZERO, evaluated with PO | 1970 B.9; 1968 B.9, same | calls PO | Maps directly |
| [FC](advancedex/FC.md) | The same pairs as COMB in another order, without recursion | 1970 B.7; 1968 B.7, same | outer product, reshape, compress, row indexing, transpose | Compress is r_eplicate; needs transpose (not yet in X_eTaL) |
| [GC](advancedex/GC.md) | The greatest common divisor of a two-item vector | 1970 B.5; 1968 B.5, same | branch loop, rotate, drop, indexing, residue reduction | Recursion; rotate, drop, indexing |
| [GCD](advancedex/GCD.md) | The greatest common divisor of two numbers (Euclid) | 1970 B.5; 1968 B.5, same | branch loop, residue | Recursion with a guard |
| [GCV](advancedex/GCV.md) | The greatest common divisor of all the items of a vector | 1970 B.5; 1968 B.5, same | branch loop, min reduction, residue, compress | Recursion; min reduction, residue, r_eplicate |
| [HILB](advancedex/HILB.md) | The Hilbert matrix of order N | 1970 B.10; 1968 B.10, same | outer product, iota, reciprocal | Maps directly: the reciprocal of an outer sum of the counts, minus 1 |
| [HTD](advancedex/HTD.md) | Hexadecimal to decimal, rejecting a non-hex digit | 1970 B.8; 1968 B.8, same | reshape (padding), index-of, decode, membership, floor, branch | i_ndexOf, d_ecode and m_ember?; the branch becomes a guard |
| [IN](advancedex/IN.md) | Where a word starts in a text, overlapping occurrences included | 1970 B.4; 1968 B.4, same | compress, iota, outer product, inner product (and-equal), indexing | Maps directly: an outer sum of indices, an all-equal reduction along rows, w_here |
| [INV](advancedex/INV.md) | Matrix inverse by Gauss-Jordan elimination, without pivoting | 1970 B.10; 1968 B.10, same | transpose, reshape, ravel, rotate on an axis, outer product, indexed assignment, branch loop | The loop becomes recursion or a reduction over the N steps, the indexed assignment a new value; matrix inverse (domino) is a feature X_eTaL still needs |
| [INVP](advancedex/INVP.md) | Gauss-Jordan matrix inverse with pivoting | 1970 B.10; 1968 B.10, same | as INV, plus max-reduce, magnitude, index-of, row swap by indexed assignment, grade up | As INV; the row swap needs a functional update (amend, on the wish list) or a ! name |
| [IN1](advancedex/IN1.md) | Like IN, but non-overlapping occurrences only | 1970 B.4; 1968 B.4, same | branch loop, membership, reduction on an axis, reshape, index-of, compress | Recursion for the loop; m_ember?, i_ndexOf, r_eshape, a reduction on axis 1 |
| [LFC](advancedex/LFC.md) | Pairs of letters: the alphabet indexed by FC N | 1970 B.7; 1968 B.7, same | indexing a character vector by a matrix | Indexing by an array of indices |
| [LOOKUP](advancedex/LOOKUP.md) | Prints the data for each name entered, or NO SUCH NAME | 1970 B.3; 1968 B.3, same | quote-quad input, computed branch, take and drop, outer product, inner product, indexing | []R_EAD, recursion, guards for the computed branch, indexing |
| [PACK](advancedex/PACK.md) | Serial number, month, day and year packed into one number | 1970 B.2; 1968 B.2, same | decode, arithmetic | d_ecode with a mixed radix; maps directly |
| [PALL](advancedex/PALL.md) | The matrix of all permutations of order N | 1970 B.11; 1968 B.11, same | factorial, reshape, indexed row assignment, branch loop, calls PERM | PERM over every count up to !N with e_ach, the rows joined into a matrix; the factorial is a product ('* r_/ r_ange) |
| [PER](advancedex/PER.md) | All permutations, built recursively, faster than PALL | 1970 B.11; 1968 B.11, same | recursion, branch loop, expand, membership, indexed column assignment, reshape, factorial | Recursion is there; needs expand (or w_here and a scatter) and a column insert; the factorial is a product |
| [PERM](advancedex/PERM.md) | The B-th permutation of order N (Woodrum's method) | 1970 B.11; 1968 B.11, same | encode (mixed radix), branch loop, indexed assignment, comparison | e_ncode is there; the loop with its scattered update becomes recursion or a fold |
| [PO](advancedex/PO.md) | Evaluates polynomials (coefficients along the first axis) at every item of X | 1970 B.6; 1968 B.6, same | outer product (power), inner product, iota | An outer product, then a sum over the first axis |
| [POL](advancedex/POL.md) | The same as POLY, by an inner product | 1970 B.6; 1968 B.6, same | inner product, iota, power | A sum of products (X_eTaL has no inner product; the sum of a product does it) |
| [POLY](advancedex/POLY.md) | Evaluates a polynomial (ascending coefficients) at a scalar | 1970 B.6; 1968 B.6, same | power, iota, ravel, shape, sum reduction | Maps directly: the sum of the coefficients times the powers of X |
| [POLYB](advancedex/POLYB.md) | The same as POLY, by base value on the reversed coefficients | 1970 B.6; 1968 B.6, same | decode, reverse | d_ecode of the reversed coefficients |
| [RESET](advancedex/RESET.md) | Empties the lists ENTER and LOOKUP use | 1970 B.3; 1968 B.3, same | multiple assignment, globals | ! names, one assignment each |
| [TIME](advancedex/TIME.md) | CPU time since its last call, in minutes, seconds and sixtieths | 1970 B.7; 1968 B.7, same | I-beam 21 (CPU time), encode, a global | Needs a clock (a quad name) and a ! name for the last reading; e_ncode is there |
| [TRUTH](advancedex/TRUTH.md) | The 2 to the N rows of N bits (a truth table's arguments) | 1970 B.4; 1968 B.4, same | outer product (divide), floor, residue, power, iota | Maps directly (or e_ncode in base 2) |
| [UNPACK](advancedex/UNPACK.md) | One number back into serial number, month, day and year | 1970 B.2; 1968 B.2, same | encode | e_ncode; maps directly |
| [ZERO](advancedex/ZERO.md) | A root of F between two points by false position, to a tolerance | 1970 B.9; 1968 B.9, same | branch loop, indexed assignment, decode, a global function, magnitude | The loop becomes recursion and the indexed assignment a new pair; F is passed as an operand |

## Features X_eTaL is known to need

- Matrix inverse and division (APL's domino), for INV and INVP (which
  are themselves Gauss-Jordan inverses, so a port could be the
  library that provides it); to be decided with the user when the
  ports reach them.
- Transpose, for BIN, FC and INV (planned: `o_\`, A2 and B2).
- An inner product, for POL, PO, IN and LOOKUP (a sum of products or
  an all-equal reduction does it meanwhile).
- A binomial (APL's dyadic `!`), for BIN; factorials are products.
- A functional update of an item or row (amend, on the wish list), for
  INV, INVP, PERM, PER and ZERO, whose loops assign into indices.
- A clock, for TIME (APL\360's I-beam 21).
- Loops by branch become recursion or reductions throughout; every
  function with a `&#8594;` loop needs that rewrite.
- Execute (reading text as a program): none of the 32 needs it.
- Nested arrays are now in the language (B14, B16), for any function
  that keeps items of different lengths.

The columns above will say which functions need which.
