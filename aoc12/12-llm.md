# Advent of Code Day 12

Please find 12-text.md in the project files for a description of today's puzzle.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today we get yet another 2D "grid", this time comprised of entirely capital letters [A-Z].
We're doing some kind of clustering, and perimeter mapping. Locating and quantifying contiguous regions and their perimeters.

Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with. Consider that the A-Z characters themselves are insignificant, only that we can group like with like, so any way of representing up to potentially 26 such entities will suffice (e.g as bytecode or whatever).

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1. Flexiable abstractions are often rewarded.

After we've decided how to represent the input data, can you generate a few unit tests for some yet unwritten primary function that will accept some input data and return an integer?

(Note: I've glanced at my puzzle input data and it's a 140x140 grid of capital letters).
