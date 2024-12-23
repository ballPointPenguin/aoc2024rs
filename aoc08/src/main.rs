use std::collections::HashMap;
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
    x: usize,
    y: usize,
}

type AntennaMap = HashMap<char, Vec<Position>>;
type AntinodeMap = HashMap<Position, Vec<char>>;

fn parse_antennas(input: &str) -> AntennaMap {
    let mut map: AntennaMap = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                map.entry(c).or_default().push(Position { x, y });
            }
        }
    }
    map
}

fn find_all_antinodes(antennas: &AntennaMap) -> AntinodeMap {
    let mut map: AntinodeMap = HashMap::new();
    for (freq, positions) in antennas {
        // This is just a placeholder, and does not calculate the antinodes
        for pos in positions {
            map.entry(*pos).or_default().push(*freq);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

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
