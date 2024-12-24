use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./08-input.txt")?;
    let result = count_antinodes(&input);
    let result2 = count_resonant_antinodes(&input);

    println!("Result: {}", result);
    println!("Result 2: {}", result2);
    Ok(())
}

pub fn count_antinodes(input: &str) -> usize {
    let antennas = parse_antennas(input);
    let is_in_bounds = make_is_in_bounds(input);
    let antinodes = find_all_antinodes(&antennas, is_in_bounds);
    antinodes.len()
}

pub fn count_resonant_antinodes(input: &str) -> usize {
    let antennas = parse_antennas(input);
    let is_in_bounds = make_is_in_bounds(input);
    let antinodes = find_resonant_antinodes(&antennas, is_in_bounds);
    antinodes.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i8,
    y: i8,
}

type AntennaMap = HashMap<char, Vec<Position>>;
type Antinodes = HashSet<Position>;

fn parse_antennas(input: &str) -> AntennaMap {
    let mut map: AntennaMap = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                map.entry(c).or_default().push(Position {
                    x: x as i8,
                    y: y as i8,
                });
            }
        }
    }
    map
}

fn pairings(antennas: &AntennaMap, freq: char) -> Vec<(Position, Position)> {
    let positions = antennas.get(&freq).unwrap();
    let mut pairs: Vec<(Position, Position)> = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            pairs.push((positions[i], positions[j]));
        }
    }

    pairs
}

fn find_all_antinodes(antennas: &AntennaMap, is_in_bounds: impl Fn(Position) -> bool) -> Antinodes {
    let mut antinodes: Antinodes = HashSet::new();

    for freq in antennas.keys() {
        let pairs = pairings(antennas, *freq);
        for (pos1, pos2) in pairs {
            for antinode in calculate_antinodes(pos1, pos2) {
                if is_in_bounds(antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes
}

fn find_resonant_antinodes(
    antennas: &AntennaMap,
    is_in_bounds: impl Fn(Position) -> bool,
) -> Antinodes {
    let mut antinodes: Antinodes = HashSet::new();

    for freq in antennas.keys() {
        let pairs = pairings(antennas, *freq);
        for pair in pairs {
            for antinode in calculate_resonant_antinodes(pair, &is_in_bounds) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes
}

fn calculate_antinodes(pos1: Position, pos2: Position) -> [Position; 2] {
    let x_diff = (pos1.x - pos2.x).abs();
    let y_diff = (pos1.y - pos2.y).abs();
    let x_min = pos1.x.min(pos2.x) - x_diff;
    let x_max = pos1.x.max(pos2.x) + x_diff;
    let y_min = pos1.y.min(pos2.y) - y_diff;
    let y_max = pos1.y.max(pos2.y) + y_diff;

    // x increases while y decreases or vice versa
    if (pos2.x > pos1.x) != (pos2.y > pos1.y) {
        [
            Position { x: x_min, y: y_max },
            Position { x: x_max, y: y_min },
        ]
    } else {
        // Default case covers:
        // - Both x and y increase/decrease together
        // - Vertical lines [pos1.x == pos2.x]
        // - Horizontal lines [pos1.y == pos2.y]
        [
            Position { x: x_min, y: y_min },
            Position { x: x_max, y: y_max },
        ]
    }
}

fn calculate_resonant_antinodes(
    pair: (Position, Position),
    is_in_bounds: impl Fn(Position) -> bool,
) -> Vec<Position> {
    let mut antinodes: Antinodes = HashSet::new();
    let (pos1, pos2) = pair;

    // Calculate direction vector and reduce to unit steps
    let dx = pos2.x - pos1.x;
    let dy = pos2.y - pos1.y;

    // Helper to extend line using the pattern
    let mut extend_line = |start: Position| {
        let mut current = start;

        while is_in_bounds(current) {
            antinodes.insert(current);
            current = Position {
                x: current.x + dx,
                y: current.y + dy,
            };
        }

        // And the other direction
        current = Position {
            x: start.x - dx,
            y: start.y - dy,
        };

        while is_in_bounds(current) {
            antinodes.insert(current);
            current = Position {
                x: current.x - dx,
                y: current.y - dy,
            };
        }
    };

    extend_line(pos1);

    antinodes.into_iter().collect()
}

fn make_is_in_bounds(input: &str) -> impl Fn(Position) -> bool {
    let height = input.lines().count() as i8;
    let width = input.lines().next().unwrap().len() as i8;

    move |pos: Position| pos.x >= 0 && pos.y >= 0 && pos.x < width && pos.y < height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_antinodes() {
        let result = calculate_antinodes(Position { x: 1, y: 2 }, Position { x: 2, y: 4 });
        let expected = [Position { x: 0, y: 0 }, Position { x: 3, y: 6 }];
        assert!(result.contains(&expected[0]) && result.contains(&expected[1]));
    }

    #[test]
    fn test_calculate_antinodes_inverted() {
        let result = calculate_antinodes(Position { x: 1, y: 4 }, Position { x: 2, y: 2 });
        let expected = [Position { x: 0, y: 6 }, Position { x: 3, y: 0 }];
        assert!(result.contains(&expected[0]) && result.contains(&expected[1]));

        let result = calculate_antinodes(Position { x: 2, y: 2 }, Position { x: 1, y: 4 });
        let expected = [Position { x: 0, y: 6 }, Position { x: 3, y: 0 }];
        assert!(result.contains(&expected[0]) && result.contains(&expected[1]));
    }

    #[test]
    fn test_calculate_antinodes_horizontal() {
        let result = calculate_antinodes(Position { x: 1, y: 2 }, Position { x: 2, y: 2 });
        let expected = [Position { x: 0, y: 2 }, Position { x: 3, y: 2 }];
        assert!(result.contains(&expected[0]) && result.contains(&expected[1]));
    }

    #[test]
    fn test_calculate_antinodes_vertical() {
        let result = calculate_antinodes(Position { x: 1, y: 2 }, Position { x: 1, y: 4 });
        let expected = [Position { x: 1, y: 0 }, Position { x: 1, y: 6 }];
        assert!(result.contains(&expected[0]) && result.contains(&expected[1]));
    }

    #[test]
    fn test_calulate_antinodes_negative() {
        let result = calculate_antinodes(Position { x: 0, y: 0 }, Position { x: 2, y: 2 });
        let expected = [Position { x: -2, y: -2 }, Position { x: 4, y: 4 }];
        assert!(result.contains(&expected[0]) && result.contains(&expected[1]));
    }

    const SAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_antennas() {
        let antennas = parse_antennas(SAMPLE_INPUT);
        assert_eq!(antennas.len(), 2);
        assert_eq!(antennas.get(&'0').unwrap().len(), 4);
        assert_eq!(antennas.get(&'A').unwrap().len(), 3);
    }

    #[test]
    fn test_pairings() {
        let antennas = parse_antennas(SAMPLE_INPUT);
        let pairs_0 = pairings(&antennas, '0');
        assert_eq!(pairs_0.len(), 6);
        let pairs_a = pairings(&antennas, 'A');
        assert_eq!(pairs_a.len(), 3);
    }

    #[test]
    fn test_sample_input() {
        // From the problem description
        assert_eq!(count_antinodes(SAMPLE_INPUT), 14);
    }

    #[test]
    fn test_out_of_bounds() {
        let input = "\
..a..
.....
..a..";
        // Antinodes would be out of bounds
        assert_eq!(count_antinodes(input), 0);
    }

    #[test]
    fn test_different_frequencies() {
        let input = "\
.....
.....
..a..
.....
..A..
.....
.....";
        // Different frequencies should not create antinodes
        assert_eq!(count_antinodes(input), 0);
    }

    #[test]
    fn test_single_pair() {
        let input = "\
.....
.....
..a..
.....
..a..
.....
.....";
        // One pair should create two antinodes
        assert_eq!(count_antinodes(input), 2);
    }

    #[test]
    fn test_horizontal_antinodes() {
        let input = "\
.......
..a.a..
.......";
        // Antinodes should be created horizontally
        assert_eq!(count_antinodes(input), 2);
    }

    #[test]
    fn test_diagonal_antinodes() {
        let input = "\
.......
.......
....a..
.......
..a....
.......
.......";
        // Antinodes should be created diagonally
        assert_eq!(count_antinodes(input), 2);
    }

    #[test]
    fn test_overlapping_antinode() {
        let input = "\
..B..
.....
..a..
.....
..a..
.....
..A..";
        // Antinodes may occur at locations that contain antennas
        assert_eq!(count_antinodes(input), 2);
    }

    #[test]
    fn test_resonant_antinodes_simple_line() {
        let input = "\
.....
..T..
..T..
..T..
.....";
        // All three T's are collinear and should be antinodes
        // Plus two more antinodes at the ends of the line
        assert_eq!(count_resonant_antinodes(input), 5);
    }

    #[test]
    fn test_resonant_antinodes_diagonal() {
        let input = "\
T....
.T...
..T..
.....
.....";
        // Three T's are collinear diagonally
        // Plus two more antinodes extending the line
        assert_eq!(count_resonant_antinodes(input), 5);
    }

    #[test]
    fn test_resonant_antinodes_multiple_lines() {
        let input = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        // Forms both vertical and horizontal lines through center T
        // Should create antinodes at all T positions plus the ends
        // of both lines (being careful not to double-count)
        assert_eq!(count_resonant_antinodes(input), 9);
    }

    #[test]
    fn test_resonant_sample_input() {
        // From the problem description
        assert_eq!(count_resonant_antinodes(SAMPLE_INPUT), 34);
    }
}
