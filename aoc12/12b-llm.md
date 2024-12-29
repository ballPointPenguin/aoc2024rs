# Advent of Code Day 12 Part 2

Please find 12-text.md in the project files for a description of today's puzzle,
followed by 12b-text.md for "Part 2" that we are going to solve together now.
Part 1 solution is in 12-main.rs. We'll continue with that file, updating with additional functions to accommodate the Part 2 requirements.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today we get yet another 2D "grid", this time comprised of entirely capital letters [A-Z].
We're doing some kind of clustering, and perimeter mapping. Locating and quantifying contiguous regions and their perimeters.

For Part 2, we need a way to calculate the `number of sides` for each region.

(Note: I've glanced at my puzzle input data and it's a 140x140 grid of capital letters).
