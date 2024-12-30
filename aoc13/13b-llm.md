# Advent of Code Day 13 Part 2

Please find 13-text.md in the project files for a description of today's puzzle.
And 13b-text.md is the update we'll be solving together.
My imlementation from Part 1, with plenty of help from Claude, is in 13-main.rs.
I've also put some related math, for contemplation, in 13-math.md.

We're working through the daily "Advent of Code" puzzles, using Rust.
Previous days puzzles and my rust implementations are now in this project as well, so feel free to catch up on where we've been the last few days, and how my Rust programming is coming along.

Today we have a unique sort of puzzle input, in the form of machine specifications described in 3 lines of text each, with a newline between each machine description.

We've got some interesting algebra today. An astute Claude Sonnet determined that the linear equations are such that there is either a valid integer solution for `a`, or there isn't. So there is no need to iterate through valid `a` values comparing for lowest "token cost".

Part 2 does not change the formula. It only attempts to add complexity by having us add 10 trillion to each target x and y value. Since we're not brute forcing anything, it's possible that our solution for Part 1 will still work, and all we need to do is write a small function that modifies the target values of each machine before passing them to the main `calulate_tokens`.

Note: I've glanced at my puzzle input data and it describes about 320 such "machines".
Their button values for x and y are all 2-digit integers.
Their target values for x and y are all 4 or 5-digit integers.
