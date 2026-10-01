# LeetCode problems in X_eTaL

LeetCode problems as X_eTaL answers them: the problem as stated, then
the array way of thinking about it, then the cousins the same arrays
answer in a line each. Each is a commented notebook file in
`demos/leetcode/`: run one with `just show demos/leetcode/NAME.xtl` to
see every line beside its output. Each is also in the live demo's Open
list, and its output is pinned by a golden
(`reg/run-leetcode-NAME`).

| Problem | The array way | Also answers | File |
| ------- | ------------- | ------------ | ---- |
| [1805. Number of Different Integers in a String](https://leetcode.com/problems/number-of-different-integers-in-a-string/) | a mask of the digits (`m_ember?`); runs start where the mask turns on, and a scan of the starts numbers them; every number at once, either by blanking the rest and reading it with `n_umbers`, or in Ints as digit times a power of 10 summed per run by a table; then `t_ally u_nique` | the largest, the sum, the numbers without repeats and in order, the digit sum, where each number starts; [1796. Second Largest Digit in a String](https://leetcode.com/problems/second-largest-digit-in-a-string/); [2042. Check if Numbers Are Ascending in a Sentence](https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/) | [numbers-in-string.xtl](../demos/leetcode/numbers-in-string.xtl) |
