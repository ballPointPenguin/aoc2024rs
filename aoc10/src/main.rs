use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./10-input.txt")?;

    let result = sum_trailhead_scores(&input);
    let result2 = sum_unique_trailhead_scores(&input);

    println!("Result: {}", result);
    println!("Result2: {}", result2);

    Ok(())
}

pub fn sum_trailhead_scores(input: &str) -> usize {
    let grid = parse_grid(input);
    let start_positions = grid.get_start_positions();

    start_positions
        .into_iter()
        .map(|start| grid.find_reachable_nines(start).len())
        .sum()
}

pub fn sum_unique_trailhead_scores(input: &str) -> usize {
    let grid = parse_grid(input);
    let start_positions = grid.get_start_positions();

    start_positions
        .into_iter()
        .map(|start| {
            grid.find_valid_paths(start)
                .values()
                .map(|paths| paths.len())
                .sum::<usize>()
        })
        .sum()
}

fn parse_grid(input: &str) -> Grid {
    let cells: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let width = cells[0].len();
    let height = cells.len();
    Grid {
        cells,
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Grid {
    cells: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get_start_positions(&self) -> HashSet<Position> {
        // Find all positions with height 0
        let mut start_positions = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == 0 {
                    start_positions.insert(Position { x, y });
                }
            }
        }
        start_positions
    }

    fn get_valid_next_positions(&self, pos: Position) -> Vec<Position> {
        // Return positions of all adjacent cells with height = current + 1
        // Check all 4 cardinal directions
        let current_height = self.cells[pos.y][pos.x];
        let target_height = current_height + 1;

        let mut valid = Vec::new();

        // Check all 4 cardinal directions
        let deltas = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in deltas {
            // Convert to signd arithmetic for bounds checking
            let new_x = pos.x as i32 + dx;
            let new_y = pos.y as i32 + dy;

            // Bounds check
            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // Height check
                if self.cells[new_y][new_x] == target_height {
                    valid.push(Position { x: new_x, y: new_y });
                }
            }
        }

        valid
    }

    fn find_reachable_nines(&self, start: Position) -> HashSet<Position> {
        // Use flood fill to find all reachable 9s from this starting position
        let mut reachable_nines = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Start the flood fill
        queue.push_back(start);
        visited.insert(start);

        while let Some(pos) = queue.pop_front() {
            // If we've reached a 9, record it
            if self.cells[pos.y][pos.x] == 9 {
                reachable_nines.insert(pos);
            }

            // Get next valid positions and continue flood fill
            for next_pos in self.get_valid_next_positions(pos) {
                if !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    queue.push_back(next_pos);
                }
            }
        }

        reachable_nines
    }

    fn find_valid_paths(&self, start: Position) -> HashMap<Position, HashSet<Vec<Position>>> {
        // First find all reachable nines from this start
        let reachable_nines = self.find_reachable_nines(start);

        let mut paths_by_end = HashMap::new();
        for end in reachable_nines {
            let mut paths = HashSet::new();
            let mut current_path = vec![start];
            self.dfs_paths(start, end, &mut current_path, &mut paths);
            paths_by_end.insert(end, paths);
        }

        paths_by_end
    }

    fn dfs_paths(
        &self,
        current: Position,
        target: Position,
        path: &mut Vec<Position>,
        paths: &mut HashSet<Vec<Position>>,
    ) {
        // If we've taken too many steps, backtrack
        if path.len() > 10 {
            return;
        }

        // If we've reached a 9 and it's our target, check if path is valid length
        if current == target && self.cells[current.y][current.x] == 9 {
            if path.len() == 10 {
                paths.insert(path.clone());
            }
            return;
        }

        // Try each valid next position
        let next_positions = self.get_valid_next_positions(current);
        for next in next_positions {
            path.push(next);
            self.dfs_paths(next, target, path, paths);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sum_trailhead_scores {
        use super::*;

        #[test]
        fn simple_vertical_path() {
            let input = "\
0
1
2
3
4
5
6
7
8
9";
            assert_eq!(sum_trailhead_scores(&input), 1); // One trailhead, one path
        }

        #[test]
        fn multiple_trailheads() {
            let input = "\
1055955
2555855
3555755
4567654
5558553
5559552
5555501";
            assert_eq!(sum_trailhead_scores(&input), 3); // Two trailheads with scores 1 and 2
        }

        #[test]
        fn example_from_puzzle() {
            let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
            assert_eq!(sum_trailhead_scores(&input), 36); // Nine trailheads
        }

        #[test]
        fn no_valid_paths() {
            let input = "\
012
901
890";
            assert_eq!(sum_trailhead_scores(&input), 0); // No valid paths possible
        }
    }

    mod sum_unique_trailhead_scores {
        use super::*;

        #[test]
        // .....0.
        // ..4321.
        // ..5..2.
        // ..6543.
        // ..7..4.
        // ..8765.
        // ..9....
        fn single_trailhead_single_peak() {
            let input = "\
1111808
1143211
1151121
1165431
1171141
1187651
1191111";
            assert_eq!(sum_unique_trailhead_scores(&input), 3);
        }

        #[test]
        // ..90..9
        // ...1.98
        // ...2..7
        // 6543456
        // 765.987
        // 876....
        // 987....
        fn single_trailhead_multiple_peaks() {
            let input = "\
1190819
1111198
1112117
6543456
7651987
8761111
9871111";
            assert_eq!(sum_unique_trailhead_scores(&input), 13);
        }

        #[test]
        // 012345
        // 123456
        // 234567
        // 345678
        // 4.6789
        // 56789.
        fn single_tailhead_max_paths() {
            let input = "\
012345
123456
234567
345678
416789
567891";
            assert_eq!(sum_unique_trailhead_scores(&input), 227);
        }

        #[test]
        fn many_trailheads_many_peaks() {
            let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
            assert_eq!(sum_unique_trailhead_scores(&input), 81);
        }
    }
}
