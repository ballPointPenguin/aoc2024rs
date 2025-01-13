# Advent of Code Day 17

Please find 17-text.md in the project files for a description of today's puzzle.
We're working through the daily "Advent of Code" puzzles, using Rust.

Today it seems like we need to implement some kind of compiler / interpreter and also use a 3-bit "computer" to solve the puzzle.

Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with.
The input contains 3 lines for the "Registers", each with an integer value. Followed by a newline.
Followed by the "Program", a list of 3-bit numbers.

I like to create a `parse_input` function that transforms the lines of input text into some data structure. It's a good place to start. Once we have some idea how to proceed, I'll begin writing unit tests.

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1. Flexible abstractions are often rewarded.
Perhaps new rules for interpreting the program will be introduced.

(Note: I've glanced at my puzzle input data and the program is merely 16 instructions long).

How shall we approach today's puzzle?
