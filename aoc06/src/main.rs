use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./06-input.txt")?;

    let result = count_guard_positions(&input);

    println!("{}", result);

    Ok(())
}

fn count_guard_positions(input: &str) -> usize {
    let (grid, start_pos) = parse_grid(input);
    let mut guard = Guard::new(start_pos, Direction::North);

    while guard.is_in_bounds(guard.next_position(), &grid) {
        // If next_position is '#', turn right, else move forward
        if position_value(&grid, guard.next_position()) == '#' {
            guard.turn_right();
        } else {
            guard.move_forward(&grid);
        }
    }

    guard.visited.len()
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .unwrap();

    (grid, start_pos)
}

fn position_value(grid: &[Vec<char>], position: (usize, usize)) -> char {
    let (x, y) = position;
    grid[y][x]
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Guard {
    position: (usize, usize),
    facing: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(position: (usize, usize), facing: Direction) -> Self {
        let mut visited = HashSet::new();
        visited.insert(position);
        Self {
            position,
            facing,
            visited,
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn move_forward(&mut self, grid: &[Vec<char>]) -> bool {
        let next = self.next_position();
        if self.is_in_bounds(next, grid) {
            self.position = next;
            self.visited.insert(next);
            true
        } else {
            false
        }
    }

    fn next_position(&self) -> (usize, usize) {
        let (x, y) = self.position;
        match self.facing {
            Direction::North => (x, y.wrapping_sub(1)),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x.wrapping_sub(1), y),
        }
    }

    fn is_in_bounds(&self, next: (usize, usize), grid: &[Vec<char>]) -> bool {
        let (x, y) = next;

        // Since we're using wrapping_sub, a "negative" position would be represented by a large positive number
        if x >= grid[0].len() || y >= grid.len() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = "\
.#.
.^.
...";

        let (grid, start_pos) = parse_grid(input);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0].len(), 3);
        assert_eq!(grid[1][1], '^');
        assert_eq!(start_pos, (1, 1));
    }

    const SAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_count_guard_positions() {
        assert_eq!(count_guard_positions(SAMPLE_INPUT), 41);
    }

    #[test]
    fn test_small_grid() {
        let input = "\
.#.
.^.
...";
        assert_eq!(count_guard_positions(input), 2);
    }

    #[test]
    fn test_immediate_exit() {
        let input = "\
^..
...
...";
        assert_eq!(count_guard_positions(input), 1);
    }

    #[test]
    fn test_guard_bounds() {
        let grid = vec![
            vec!['.', '.', '.'],
            vec!['.', '^', '.'],
            vec!['.', '.', '.'],
        ];
        let mut guard = Guard::new((1, 1), Direction::North);

        // Moving up is in bounds
        guard.move_forward(&grid);
        assert!(guard.is_in_bounds(guard.position, &grid));

        // Moving up again would be out of bounds
        assert!(!guard.is_in_bounds(guard.next_position(), &grid));

        // Turn right and move east -> in bounds
        guard.turn_right();
        guard.move_forward(&grid);
        assert!(guard.is_in_bounds(guard.position, &grid));

        // Move east again -> out of bounds
        assert!(!guard.is_in_bounds(guard.next_position(), &grid));
    }
}
