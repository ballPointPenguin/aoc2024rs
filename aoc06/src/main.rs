use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

// First iteration: 89.5s
// Second iteration: 88.5s

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let input = read_to_string("./06-input.txt")?;

    let result = count_guard_positions(&input);
    let result2 = count_possible_loop_positions(&input);

    println!("Result: {}", result);
    println!("Result2: {}", result2);

    let end = Instant::now();
    println!("Time taken: {:?}", end.duration_since(start));

    Ok(())
}

fn count_guard_positions(input: &str) -> usize {
    let grid = Grid::new(input);
    let guard = Guard::new(grid.start_pos(), Direction::North);

    let (count, _) = simulate_guard_path(&grid, guard);

    count
}

fn simulate_guard_path(grid: &Grid, mut guard: Guard) -> (usize, bool) {
    let mut loop_detected = false;

    while guard.is_in_bounds(guard.next_position(), &grid) {
        // If next_position is '#', turn right, else move forward
        if grid.get(guard.next_position()) == Some('#') {
            guard.turn_right();
        } else {
            guard.move_forward(&grid);
        }

        // If we've already seen this state, we're in a loop
        if guard.states.contains(&GuardState {
            position: guard.position,
            facing: guard.facing,
        }) {
            loop_detected = true;
            break;
        }

        // Add the current state to the set of states
        guard.states.insert(GuardState {
            position: guard.position,
            facing: guard.facing,
        });
    }

    (guard.visited.len(), loop_detected)
}

fn count_possible_loop_positions(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let guard = Guard::new(grid.start_pos(), Direction::North);
    let mut loop_count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = (x, y);

            // Skip invalid positions
            if pos == grid.start_pos() || grid.get(pos) == Some('#') {
                continue;
            }

            grid.place_obstacle(pos);

            if let (_, true) = simulate_guard_path(&grid, guard.clone()) {
                loop_count += 1;
                println!("Loop count now: {}", loop_count);
            }

            grid.remove_obstacle(pos);
        }
    }

    loop_count
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len();
        let width = cells[0].len();
        Self {
            cells,
            height,
            width,
        }
    }

    fn get(&self, pos: (usize, usize)) -> Option<char> {
        let (x, y) = pos;
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    fn place_obstacle(&mut self, pos: (usize, usize)) {
        let (x, y) = pos;
        if y < self.height && x < self.width {
            self.cells[y][x] = '#';
        }
    }

    fn remove_obstacle(&mut self, pos: (usize, usize)) {
        let (x, y) = pos;
        if y < self.height && x < self.width {
            self.cells[y][x] = '.';
        }
    }

    fn start_pos(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardState {
    position: (usize, usize),
    facing: Direction,
}

#[derive(Debug, Clone)]
struct Guard {
    position: (usize, usize),
    facing: Direction,
    visited: HashSet<(usize, usize)>,
    states: HashSet<GuardState>,
}

impl Guard {
    fn new(position: (usize, usize), facing: Direction) -> Self {
        let mut visited = HashSet::new();
        let mut states = HashSet::new();
        visited.insert(position);
        states.insert(GuardState { position, facing });
        Self {
            position,
            facing,
            visited,
            states,
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

    fn move_forward(&mut self, grid: &Grid) -> bool {
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

    fn is_in_bounds(&self, next: (usize, usize), grid: &Grid) -> bool {
        let (x, y) = next;

        if x >= grid.width || y >= grid.height {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_grid() {
        let input = "\
.#.
.^.
...";

        let grid = Grid::new(input);

        assert_eq!(grid.height, 3);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.get((1, 2)), Some('.'));
        assert_eq!(grid.start_pos(), (1, 1));
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
        let input = "\
...
.^.
...";

        let grid = Grid::new(input);
        let mut guard = Guard::new(grid.start_pos(), Direction::North);

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
