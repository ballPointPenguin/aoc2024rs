# Advent of Code Day 16

Please find 16-text.md in the project files for a description of today's puzzle.
We're working through the daily "Advent of Code" puzzles, using Rust.

Today we get yet another 2D "grid", this time comprised of characters [.#SE]. (Just one 'S' and one 'E').
This is a classic maze. The goal is to find the "cheapest" path from S to E. Each step costs 1 point, but each 90 degree turn costs 1000 points.

Can you relate any CompSci lore or familiar algorithms or classical problems to today's challenge?

We should consider how to represent the input data in a way that is easy to work with. Consider that the '#', 'S', and 'E' locations are static, but we will need to track some paths or positions for our "Reindeer".
Our maze representation could be a 'grid' of chars (`Vec<Vec<char>>` or similar), but since we are mainly concerned with '#' and '.' it could reasonably be a binary representation, with the 'S' and 'E' coordinates saved elsewhere. Or it could be some other abstraction. Consider we want efficient pathfinding iterations.

I like to create a `parse_input` function that transforms the lines of input text into some data structure. It's a good place to start. Once we have some idea how to proceed, I'll begin writing unit tests.

And, for what it's worth, we can try to anticipate that in Part 2, the puzzle author usually throws a curveball at us with the tendency to break the assumptions we've made in Part 1. Flexible abstractions are often rewarded.
Perhaps the scoring will change, or some rules of motion. The input remains the same.

(Note: I've glanced at my puzzle input data and it's a 141x141 grid).

How shall we approach today's puzzle?
