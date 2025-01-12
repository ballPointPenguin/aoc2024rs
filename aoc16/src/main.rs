use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../16-input.txt");
    let maze = Maze::new(input);

    // Part 1
    if let Some((cost, _)) = maze.find_all_optimal_paths() {
        println!("Shortest path cost: {}", cost);
    }

    // Part 2
    println!(
        "Tiles in optimal paths: {}",
        maze.count_optimal_path_tiles()
    );
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    pos: Position,
    dir: Direction,
}

impl State {
    fn next_states(&self, maze: &Maze) -> Vec<(State, usize)> {
        let mut states = Vec::new();

        // Add turns (cost 1000)
        states.push((
            State {
                pos: self.pos,
                dir: self.dir.turn_left(),
            },
            1000,
        ));
        states.push((
            State {
                pos: self.pos,
                dir: self.dir.turn_right(),
            },
            1000,
        ));

        // Add forward move (cost 1)
        let (dx, dy) = self.dir.delta();
        let next_pos = Position {
            x: self.pos.x.wrapping_add(dx as usize),
            y: self.pos.y.wrapping_add(dy as usize),
        };

        if !maze.is_wall(next_pos) {
            states.push((
                State {
                    pos: next_pos,
                    dir: self.dir,
                },
                1,
            ));
        }

        states
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct OrderedState {
    cost: usize,
    state: State,
}

// Add a new struct to track paths
#[derive(Debug, Clone)]
struct PathState {
    cost: usize,
    state: State,
    path: Vec<Position>,
}

// Implement Ord so BinaryHeap becomes a min-heap (be negating comparison)
impl Ord for OrderedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.state.pos.x.cmp(&other.state.pos.x))
            .then_with(|| self.state.pos.y.cmp(&other.state.pos.y))
    }
}

impl PartialOrd for OrderedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type VisitedMap = HashMap<Position, bool>;

#[derive(Debug, Clone)]
struct Maze {
    height: usize,
    width: usize,
    start: Position,
    end: Position,
    grid: Vec<Vec<char>>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let grid = parse_input(input);
        let height = grid.len();
        let width = grid[0].len();

        let mut start = Position { x: 0, y: 0 };
        let mut end = Position { x: 0, y: 0 };

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                match cell {
                    'S' => start = Position { x, y },
                    'E' => end = Position { x, y },
                    _ => continue,
                }
            }
        }

        Self {
            height,
            width,
            start,
            end,
            grid,
        }
    }

    fn is_wall(&self, pos: Position) -> bool {
        if pos.y >= self.height || pos.x >= self.width {
            return true;
        }

        self.grid[pos.y][pos.x] == '#'
    }

    fn find_all_optimal_paths(&self) -> Option<(usize, VisitedMap)> {
        // First find the optimal cost
        let optimal_cost = self.find_shortest_path()?;

        let mut queue = Vec::new();
        let mut seen = HashMap::new();
        let mut optimal_paths = HashMap::new();

        let initial_state = State {
            pos: self.start,
            dir: Direction::East,
        };

        queue.push(PathState {
            cost: 0,
            state: initial_state,
            path: vec![self.start],
        });

        seen.insert(initial_state, 0);

        while let Some(PathState { cost, state, path }) = queue.pop() {
            // Skip paths that exceed optimal cost
            if cost > optimal_cost {
                continue;
            }

            // If we've reached the end with optimal cost, mark the entire path
            if state.pos.x == self.end.x && state.pos.y == self.end.y {
                if cost == optimal_cost {
                    for pos in path {
                        optimal_paths.insert(pos, true);
                    }
                }
                continue;
            }

            // Try all possible next states
            for (next_state, move_cost) in state.next_states(self) {
                let new_cost = cost + move_cost;

                // Only explore paths that could still achieve optimal cost
                if new_cost <= optimal_cost {
                    // Allow revisiting states if we can reach them with same cost
                    if !seen.contains_key(&next_state) || new_cost <= seen[&next_state] {
                        seen.insert(next_state, new_cost);
                        let mut new_path = path.clone();
                        new_path.push(next_state.pos);
                        queue.push(PathState {
                            cost: new_cost,
                            state: next_state,
                            path: new_path,
                        });
                    }
                }
            }
        }

        Some((optimal_cost, optimal_paths))
    }

    fn count_optimal_path_tiles(&self) -> usize {
        if let Some((_, optimal_paths)) = self.find_all_optimal_paths() {
            optimal_paths.len()
        } else {
            0
        }
    }

    fn find_shortest_path(&self) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        let mut seen = HashMap::new();

        // Start facing East
        let initial_state = State {
            pos: self.start,
            dir: Direction::East,
        };

        queue.push(OrderedState {
            cost: 0,
            state: initial_state,
        });

        seen.insert(initial_state, 0);

        while let Some(OrderedState { cost, state }) = queue.pop() {
            // If we've reached the end, return the cost
            if state.pos.x == self.end.x && state.pos.y == self.end.y {
                return Some(cost);
            }

            // If we've seen this state with a lower cost, skip it
            if let Some(&prev_cost) = seen.get(&state) {
                if prev_cost < cost {
                    continue;
                }
            }

            // Try all possible next states
            for (next_state, move_cost) in state.next_states(self) {
                let new_cost = cost + move_cost;

                // If we haven't seen this state, or if this is cheaper
                if !seen.contains_key(&next_state) || new_cost < seen[&next_state] {
                    seen.insert(next_state, new_cost);
                    queue.push(OrderedState {
                        cost: new_cost,
                        state: next_state,
                    });
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SECOND_EXAMPLE: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_parse_input() {
        let input = "S.#\nE.#";
        let maze = Maze::new(input);
        assert_eq!(maze.height, 2);
        assert_eq!(maze.width, 3);
    }

    #[test]
    fn test_simple_maze() {
        let maze = Maze::new(FIRST_EXAMPLE);
        assert_eq!(maze.find_shortest_path(), Some(7036));
    }

    #[test]
    fn test_second_example() {
        let maze = Maze::new(SECOND_EXAMPLE);
        assert_eq!(maze.find_shortest_path(), Some(11048));
    }

    #[test]
    fn test_optimal_path_tiles() {
        let maze = Maze::new(FIRST_EXAMPLE);
        assert_eq!(maze.count_optimal_path_tiles(), 45);
    }

    #[test]
    fn test_second_example_optimal_path_tiles() {
        let maze = Maze::new(SECOND_EXAMPLE);
        assert_eq!(maze.count_optimal_path_tiles(), 64);
    }
}
