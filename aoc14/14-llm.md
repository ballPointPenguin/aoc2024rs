# Advent of Code Day 14

Please find 14-text.md in the project files for a description of today's puzzle.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today we have the concept of a 2D grid, but unlike many previous days, the puzzle input itself is not an ASCII grid of text. Instead it's many lines of 2 pairs of integers. The first pair (p) range from 0 to 102. The second pair (v) range from -99 to 99.

This puzzle seems to be about basic algebra with some wrapping arithmatic. I think we can leverage some rust number wrapping conventions.

Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with.
There is a 101x103 "grid". But in the example (and test) data the grid is smaller. I suppose the grid dimensions should be configurable on creation. It's also possible that we don't even need a Grid as such, if state is confined to the robots themselves.

Then we have robots. A robot has a position (x,y), and a velocity (x,y). They don't interact with anything. If the grid dimensions are known, a robot's future position in x seconds can easily be calculated in parallel.

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1  (e.g. robot interaction, conditional velocity, etc.). Flexiable abstractions are often rewarded.

After we've decided how to represent the input data, can you generate a few unit tests for some yet unwritten primary function that will accept some input data and return an integer?

(Note: I've glanced at my puzzle input data and it's 500 robots.)
