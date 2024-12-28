# Advent of Code Day 11

Please find 11-text.md in the project files for a description of today's puzzle.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today the input data simply a sequence of 8 integers. But it's a little weird.
Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with. Consider that we will be manipulating a linear sequence of integers, inserting new integers into arbitrary positions, and eventually counting the sequence length. Both the sequence length and the size of the individual integers have the potential to grow quite large.

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1. Flexiable abstractions are often rewarded.

After we've decided how to represent the input data, can you generate a few unit tests for some yet unwritten primary function that will accept some input and return an integer?

(Note: I've glanced at my puzzle input data and it's a single line of 8 integers, ranging in size from 1 to 7 digits each).
