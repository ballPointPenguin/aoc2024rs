# Advent of Code Day 15

Please find 15-text.md in the project files for a description of today's puzzle.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today we get yet another 2D "grid", this time comprised of characters [O.#@]. (Just one '@').
This is followed by many lines of [^>V<] characters.
This seems like a kind of cellular automata, almost like game design, wherein a long sequence of "moves" will influence the state of the grid, according to a few rules about how the "world" works.

Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with. Consider that the '#' locations are static, and only the 'O' and '@' can be moved. The "moves" could be collected into some vector, to be read sequentially, and perhaps represented in some efficient way (eg binary with 00, 01, 10, 11 to indicate the 4 directions), or simply a vec of enum.

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1. Flexible abstractions are often rewarded. Perhaps the rules of the "world" will change in some way.

After we've decided how to represent the input data, can you generate a few unit tests for some yet unwritten primary function that will accept some 'Grid' and 'moves' and return an integer (sum of coordinates)?

(Note: I've glanced at my puzzle input data and it's a 50x50 grid followed by many lines of many moves).
