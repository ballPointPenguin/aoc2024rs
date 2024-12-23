use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./08-input.txt")?;
    let result = count_antinodes(&input);

    println!("Result: {}", result);

    Ok(())
}

pub fn count_antinodes(input: &str) -> usize {
    let antennas = parse_antennas(input);
    let antinodes = find_all_antinodes(&antennas);
    antinodes.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i8,
    y: i8,
}

type AntennaMap = HashMap<char, Vec<Position>>;
type AntinodeMap = HashMap<Position, Vec<char>>;

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

fn find_all_antinodes(antennas: &AntennaMap) -> AntinodeMap {
    let mut map: AntinodeMap = HashMap::new();

    for freq in antennas.keys() {
        let pairs = pairings(antennas, *freq);
        for (pos1, pos2) in pairs {
            map.entry(pos1).or_default().push(*freq);
            map.entry(pos2).or_default().push(*freq);
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

fn calculate_antinodes(pos1: Position, pos2: Position) -> (Position, Position) {
    let x_diff = (pos1.x - pos2.x).abs();
    let y_diff = (pos1.y - pos2.y).abs();
    let x_min = pos1.x.min(pos2.x) - x_diff;
    let x_max = pos1.x.max(pos2.x) + x_diff;
    let y_min = pos1.y.min(pos2.y) - y_diff;
    let y_max = pos1.y.max(pos2.y) + y_diff;

    // x increases while y decreases or vice versa
    if (pos2.x > pos1.x) != (pos2.y > pos1.y) {
        (
            Position { x: x_min, y: y_max },
            Position { x: x_max, y: y_min },
        )
    } else {
        // Default case covers:
        // - Both x and y increase/decrease together
        // - Vertical lines (pos1.x == pos2.x)
        // - Horizontal lines (pos1.y == pos2.y)
        (
            Position { x: x_min, y: y_min },
            Position { x: x_max, y: y_max },
        )
    }
}

// TODO: Needs to know the upper bounds of the grid
fn is_in_bounds(pos: Position) -> bool {
    pos.x >= 0 && pos.y >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_antinodes() {
        let (pos1, pos2) = calculate_antinodes(Position { x: 1, y: 2 }, Position { x: 2, y: 4 });
        assert_eq!(pos1, Position { x: 0, y: 0 });
        assert_eq!(pos2, Position { x: 3, y: 6 });
    }

    #[test]
    fn test_calculate_antinodes_inverted() {
        let result = calculate_antinodes(Position { x: 1, y: 4 }, Position { x: 2, y: 2 });
        let expected = HashSet::from([Position { x: 0, y: 6 }, Position { x: 3, y: 0 }]);
        assert_eq!(HashSet::from([result.0, result.1]), expected);

        let result = calculate_antinodes(Position { x: 2, y: 2 }, Position { x: 1, y: 4 });
        let expected = HashSet::from([Position { x: 0, y: 6 }, Position { x: 3, y: 0 }]);
        assert_eq!(HashSet::from([result.0, result.1]), expected);
    }

    #[test]
    fn test_calculate_antinodes_horizontal() {
        let result = calculate_antinodes(Position { x: 1, y: 2 }, Position { x: 2, y: 2 });
        let expected = HashSet::from([Position { x: 0, y: 2 }, Position { x: 3, y: 2 }]);
        assert_eq!(HashSet::from([result.0, result.1]), expected);
    }

    #[test]
    fn test_calculate_antinodes_vertical() {
        let result = calculate_antinodes(Position { x: 1, y: 2 }, Position { x: 1, y: 4 });
        let expected = HashSet::from([Position { x: 1, y: 0 }, Position { x: 1, y: 6 }]);
        assert_eq!(HashSet::from([result.0, result.1]), expected);
    }

    #[test]
    fn test_calulate_antinodes_negative() {
        let result = calculate_antinodes(Position { x: 0, y: 0 }, Position { x: 2, y: 2 });
        let expected = HashSet::from([Position { x: -2, y: -2 }, Position { x: 4, y: 4 }]);
        assert_eq!(HashSet::from([result.0, result.1]), expected);
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
}
