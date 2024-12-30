# Advent of Code Day 13

Please find 13-text.md in the project files for a description of today's puzzle.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today we have a unique sort of puzzle input, in the form of machine specifications described in 3 lines of text each, with a newline between each machine description.

This puzzle seems to be a bit of algebra. Perhaps even polynomial factoring.
Something like `a(cx) + b(dx) = X' and a(ey) + b(fy) = Y'`, where `p = 3a + b`. Positive integers are given for values c, d, e, f, X', and Y'. Find the combination of a and b that yields the lowest value p while satisfying the formula. (Please rephrase this in more conventional math terms if you can).

Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with.
Each "paragraph" of input text has 6 significant integers to parse: X & Y values for button A, X & Y values for button B, and target X & Y values.

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1. Flexiable abstractions are often rewarded.

After we've decided how to represent the input data, can you generate a few unit tests for some yet unwritten primary function that will accept some input data and return an integer?

(Note: I've glanced at my puzzle input data and it describes about 320 such "machines").
