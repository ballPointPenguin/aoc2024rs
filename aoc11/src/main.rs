use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let input = read_to_string("./11-input.txt")?;
    let input_numbers = parse_input(&input);

    let result = blink_n_times(&input_numbers, 25).len();
    println!("Result: {}", result);

    let predictor = BlinkLengthPredictor::new();
    let result2: usize = input_numbers
        .iter()
        .map(|&num| predictor.predict_length(num, 75))
        .sum();
    println!("Result2: {}", result2);

    let end = Instant::now();
    println!("Time taken: {:?}", end.duration_since(start));

    Ok(())
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn _print_transitions(transitions: &StateTransitions) {
    for (&from, to_states) in transitions.iter().sorted() {
        println!("{:7} -> {:?}", from, to_states);
    }
}

pub fn blink_n_times(input: &[u64], n: u64) -> Vec<u64> {
    let mut stones = input.to_vec();
    for _ in 0..n {
        stones = blink(&stones);
    }
    stones
}

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut new_stones = Vec::with_capacity(stones.len() * 2);

    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if has_even_digits(stone) {
            let (left, right) = split_number(stone);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }

    new_stones
}

fn has_even_digits(stone: u64) -> bool {
    stone.to_string().len() % 2 == 0
}

fn split_number(n: u64) -> (u64, u64) {
    let s = n.to_string();
    let mid = s.len() / 2;

    let left = s[..mid].parse().unwrap();
    let right = s[mid..].parse().unwrap();

    (left, right)
}

struct BlinkLengthPredictor {
    // Maps (stone, num_blinks) -> resulting sequence length
    cached_lengths: HashMap<(u64, u8), usize>,
    // The set of 54 stable states we discovered
    stable_states: HashSet<u64>,
}

type StateTransitions = HashMap<u64, Vec<u64>>;

impl BlinkLengthPredictor {
    fn new() -> Self {
        let stable_states = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 20, 24, 26, 28, 32, 36, 40, 48, 56, 57, 60, 67, 72, 77,
            80, 84, 86, 91, 94, 96, 2024, 2048, 2457, 2608, 2867, 2880, 3277, 3686, 4048, 6032,
            6072, 8096, 9184, 9456, 10120, 12144, 14168, 16192, 18216, 20482880, 24579456,
            28676032, 32772608, 36869184,
        ]
        .into_iter()
        .collect();

        let mut predictor = Self {
            cached_lengths: HashMap::new(),
            stable_states,
        };
        predictor.precalculate_lengths(75);
        predictor
    }

    fn build_transition_map(&self) -> StateTransitions {
        let mut transitions = HashMap::new();

        for &state in &self.stable_states {
            let next_states = blink(&vec![state]);
            // All resulting stones should be stable states
            debug_assert!(next_states.iter().all(|s| self.stable_states.contains(s)));
            transitions.insert(state, next_states);
        }

        transitions
    }

    fn precalculate_lengths(&mut self, iterations: u8) {
        let transitions = self.build_transition_map();

        // Start with lengths after 1 blink
        for (&state, next_states) in &transitions {
            self.cached_lengths.insert((state, 1), next_states.len());
        }

        // For each subsequent blink, use previous results
        for blinks in 2..=iterations {
            for &state in &self.stable_states {
                let next_states = &transitions[&state];
                let total_length: usize = next_states
                    .iter()
                    .map(|&s| self.cached_lengths[&(s, blinks - 1)])
                    .sum();
                self.cached_lengths.insert((state, blinks), total_length);
            }
        }
    }

    fn predict_length(&self, stone: u64, blinks: u8) -> usize {
        // If no blinks left, just return 1 (current stone)
        if blinks == 0 {
            return 1;
        }

        // If stone is in stable states, use cached result
        if self.stable_states.contains(&stone) {
            return self.cached_lengths[&(stone, blinks)];
        }

        // Otherwise, do one blink and recurse
        let next_stones = blink(&vec![stone]);
        next_stones
            .iter()
            .map(|&s| self.predict_length(s, blinks - 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_blink() {
        let input = vec![0, 1, 10, 99, 999];
        let expected = vec![1, 2024, 1, 0, 9, 9, 2021976];
        assert_eq!(blink_n_times(&input, 1).len(), expected.len());
    }

    #[test]
    fn test_sequence_growth() {
        let input = vec![125, 17];
        // After 6 blinks should have 22 stones
        assert_eq!(blink_n_times(&input, 6).len(), 22);
    }

    #[test]
    fn test_example_25_blinks() {
        let input = vec![125, 17];
        assert_eq!(blink_n_times(&input, 25).len(), 55312);
    }
}
