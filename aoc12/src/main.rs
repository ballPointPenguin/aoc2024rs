use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let input = read_to_string("./12-input.txt")?;
    let (result, result2) = calculate_price(&input);

    println!("Result: {}", result);
    println!("Result 2: {}", result2);

    let end = Instant::now();
    println!("Time taken: {:?}", end.duration_since(start));
    Ok(())
}

pub fn calculate_price(input: &str) -> (u64, u64) {
    let start = Instant::now();
    let grid = Grid::new(parse_input(input));
    let regions = find_all_regions(&grid);
    let find_regions_end = Instant::now();

    println!(
        "Finding regions took {:?}",
        find_regions_end.duration_since(start)
    );
    println!("Found {} regions", regions.len());
    println!(
        "Largest region size: {}",
        regions.iter().map(|r| r.len()).max().unwrap()
    );

    let calculate_price_start = Instant::now();
    let result = regions
        .iter()
        .map(|region| {
            let area = region.len() as u64;
            let perimeter = grid.calculate_perimeter(region) as u64;
            area * perimeter
        })
        .sum();
    let calculate_price_end = Instant::now();
    println!(
        "Calculating price took {:?}",
        calculate_price_end.duration_since(calculate_price_start)
    );

    let calculate_price_2_start = Instant::now();
    let result_2 = regions
        .iter()
        .map(|region| {
            let area = region.len() as u64;
            let sides = grid.calculate_sides(region) as u64;
            area * sides
        })
        .sum();
    let calculate_price_2_end = Instant::now();
    println!(
        "Calculating price 2 took {:?}",
        calculate_price_2_end.duration_since(calculate_price_2_start)
    );

    (result, result_2)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
    neighbors_cache: Vec<Vec<Vec<Position>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    start: Position,
    edge_type: EdgeType,
    is_inner: bool,
}

impl Grid {
    fn new(input: Vec<Vec<u8>>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let cells = input.into_iter().flatten().collect();
        let mut neighbors_cache = vec![vec![Vec::with_capacity(4); width]; height];

        for y in 0..height {
            for x in 0..width {
                let pos = Position { x, y };
                neighbors_cache[y][x] = get_neighbors_uncached(width, height, pos);
            }
        }

        Self {
            height,
            width,
            cells,
            neighbors_cache,
        }
    }

    fn get_neighbors(&self, pos: Position) -> &[Position] {
        &self.neighbors_cache[pos.y][pos.x]
    }

    fn get_cell(&self, pos: Position) -> u8 {
        self.cells[pos.y * self.width + pos.x]
    }

    fn calculate_perimeter(&self, region: &FxHashSet<Position>) -> usize {
        let mut perimeter = 0;
        for &pos in region {
            let neighbors = self.get_neighbors(pos);

            // Grid boundaries = 4 - neighbors.len()
            perimeter += 4 - neighbors.len();

            // Check all 4 cardinal directions
            for neighbor in neighbors {
                // Edge of grid or different type = perimeter edge
                if !region.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn calculate_sides(&self, region: &FxHashSet<Position>) -> usize {
        let edges = self.collect_edges(region);

        // Group edges by edge_type and inner/outer state using a HashMap
        let mut edge_groups: FxHashMap<(EdgeType, bool), Vec<Edge>> = FxHashMap::default();
        for edge in edges {
            edge_groups
                .entry((edge.edge_type, edge.is_inner))
                .or_default()
                .push(edge);
        }

        // Helper function to count sides in a group of edges
        fn count_sides(edges: &[Edge]) -> usize {
            if edges.is_empty() {
                return 0;
            }

            // Determine if we're dealing with horizontal or vertical edges
            let is_horizontal = matches!(edges[0].edge_type, EdgeType::Top | EdgeType::Bottom);

            // Group edges by their fixed coordinate (y for horizontal, x for vertical)
            let by_line: FxHashMap<usize, Vec<&Edge>> =
                edges.iter().fold(FxHashMap::default(), |mut acc, edge| {
                    let key = if is_horizontal {
                        edge.start.y
                    } else {
                        edge.start.x
                    };
                    acc.entry(key).or_default().push(edge);
                    acc
                });

            by_line
                .values()
                .map(|line_edges| {
                    let mut sorted_edges = line_edges.to_vec();
                    // Sort by the varying coordinate (x for horizontal, y for vertical)
                    sorted_edges.sort_by_key(|e| if is_horizontal { e.start.x } else { e.start.y });

                    // Count segments using windows
                    1 + sorted_edges
                        .windows(2)
                        .filter(|window| {
                            let [e1, e2] = window else { unreachable!() };
                            let (pos1, pos2) = if is_horizontal {
                                (e1.start.x, e2.start.x)
                            } else {
                                (e1.start.y, e2.start.y)
                            };

                            pos1.abs_diff(pos2) > 1
                                || (is_horizontal && e1.start.y != e2.start.y)
                                || (!is_horizontal && e1.start.x != e2.start.x)
                        })
                        .count()
                })
                .sum()
        }

        // Calculate sides for all edge groups and sum them
        edge_groups.values().map(|edges| count_sides(edges)).sum()
    }

    fn collect_edges(&self, region: &FxHashSet<Position>) -> FxHashSet<Edge> {
        type PosCompare = fn(&Position, &Position) -> bool;

        let edge_checks: [(EdgeType, PosCompare); 4] = [
            (EdgeType::Top, |n, p| n.y < p.y),
            (EdgeType::Bottom, |n, p| n.y > p.y),
            (EdgeType::Left, |n, p| n.x < p.x),
            (EdgeType::Right, |n, p| n.x > p.x),
        ];

        region
            .iter()
            .flat_map(|&pos| {
                let neighbors = self.get_neighbors(pos);

                edge_checks.iter().filter_map(move |&(edge_type, check)| {
                    // If no neighbor in this direction is part of the region
                    if !neighbors
                        .iter()
                        .any(|n| check(n, &pos) && region.contains(n))
                    {
                        Some(Edge {
                            start: pos,
                            edge_type,
                            // But if there is a neighbor in this direction (even outside region)
                            is_inner: neighbors.iter().any(|n| check(n, &pos)),
                        })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

fn find_region(grid: &Grid, start: Position) -> FxHashSet<Position> {
    let mut region =
        FxHashSet::with_capacity_and_hasher(grid.width * grid.height / 4, Default::default());
    let mut visited = vec![vec![false; grid.width]; grid.height];
    let mut queue = VecDeque::with_capacity(grid.width.max(grid.height));

    let value = grid.get_cell(start);
    queue.push_back(start);
    visited[start.y][start.x] = true;
    region.insert(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in grid.get_neighbors(current) {
            if !visited[neighbor.y][neighbor.x] && grid.get_cell(*neighbor) == value {
                visited[neighbor.y][neighbor.x] = true;
                region.insert(*neighbor);
                queue.push_back(*neighbor);
            }
        }
    }

    region
}

fn get_neighbors_uncached(width: usize, height: usize, pos: Position) -> Vec<Position> {
    let mut neighbors = Vec::with_capacity(4);
    for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let new_x = pos.x as i32 + dx;
        let new_y = pos.y as i32 + dy;
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            neighbors.push(Position {
                x: new_x as usize,
                y: new_y as usize,
            });
        }
    }
    neighbors
}

fn find_all_regions(grid: &Grid) -> Vec<FxHashSet<Position>> {
    let visited: Arc<Mutex<FxHashSet<Position>>> = Arc::new(Mutex::new(FxHashSet::default()));
    let positions: Vec<_> = (0..grid.height)
        .flat_map(|y| (0..grid.width).map(move |x| Position { x, y }))
        .collect();

    positions
        .par_iter()
        .filter_map(|&pos| {
            let mut visited = visited.lock().unwrap();
            if !visited.contains(&pos) {
                let region = find_region(grid, pos);
                visited.extend(&region);
                Some(region)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::ThreadPoolBuilder;

    fn with_fixed_threads<T: Send>(test: impl FnOnce() -> T + Send) -> T {
        let pool = ThreadPoolBuilder::new()
            .num_threads(2) // Or any fixed number
            .build()
            .unwrap();
        pool.install(test)
    }

    mod collect_edges {
        use super::*;

        #[test]
        fn simple_square() {
            let grid = Grid::new(parse_input(
                "\
AAAA
AAAA
AAAA
AAAA",
            ));
            let region = find_region(&grid, Position { x: 0, y: 0 });
            assert_eq!(grid.collect_edges(&region).len(), 16);
        }
    }

    mod calculate_sides {
        use super::*;

        #[test]
        fn simple_square() {
            let grid = Grid::new(parse_input(
                "\
AAAA
AAAA
AAAA
AAAA",
            ));
            let region = find_region(&grid, Position { x: 0, y: 0 });
            assert_eq!(grid.calculate_sides(&region), 4); // Just 4 sides for a square
        }

        #[test]
        fn e_shaped_region() {
            let grid = Grid::new(parse_input(
                "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
            ));
            let regions = find_all_regions(&grid);
            // Find the E region (should be the largest)
            let e_region = regions.iter().max_by_key(|r| r.len()).unwrap();
            assert_eq!(grid.calculate_sides(e_region), 12);
        }

        #[test]
        fn region_with_hole() {
            let grid = Grid::new(parse_input(
                "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
            ));
            let regions = find_all_regions(&grid);
            // A region should have 12 sides (4 outer + 8 inner)
            let a_region = regions.iter().max_by_key(|r| r.len()).unwrap();
            assert_eq!(grid.calculate_sides(a_region), 12);
        }
    }

    mod calculate_perimeter {
        use super::*;

        #[test]
        fn simple_square() {
            let grid = Grid::new(parse_input(
                "\
AAAA
AAAA
AAAA
AAAA",
            ));
            let region = find_region(&grid, Position { x: 0, y: 0 });
            assert_eq!(grid.calculate_perimeter(&region), 16);
        }

        #[test]
        fn multiple_regions() {
            with_fixed_threads(|| {
                let grid = Grid::new(parse_input(
                    "\
AAAA
BBCD
BBCC
EEEC",
                ));
                let regions = find_all_regions(&grid);
                assert_eq!(grid.calculate_perimeter(&regions[0]), 10); // A
                assert_eq!(grid.calculate_perimeter(&regions[1]), 8); // B
                assert_eq!(grid.calculate_perimeter(&regions[2]), 10); // C
                assert_eq!(grid.calculate_perimeter(&regions[3]), 4); // D
                assert_eq!(grid.calculate_perimeter(&regions[4]), 8); // E
            });
        }

        #[test]
        fn region_with_holes() {
            with_fixed_threads(|| {
                let grid = Grid::new(parse_input(
                    "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
                ));
                let regions = find_all_regions(&grid);
                assert_eq!(grid.calculate_perimeter(&regions[0]), 36); // O
                assert_eq!(grid.calculate_perimeter(&regions[1]), 4); // X1
                assert_eq!(grid.calculate_perimeter(&regions[2]), 4); // X2
                assert_eq!(grid.calculate_perimeter(&regions[3]), 4); // X3
                assert_eq!(grid.calculate_perimeter(&regions[4]), 4); // X4
            });
        }
    }

    mod find_regions {
        use super::*;

        #[test]
        fn simple_square() {
            let grid = Grid::new(parse_input(
                "\
AAAA
AAAA
AAAA
AAAA",
            ));
            let regions = find_all_regions(&grid);
            assert_eq!(regions.len(), 1);
            assert_eq!(regions[0].len(), 16);
        }

        #[test]
        fn test_find_region() {
            let grid = Grid::new(parse_input(
                "\
AAAA
BBCD
BBCC
EEEC",
            ));

            // Starting from (0,0), should find all 'A's
            let region = find_region(&grid, Position { x: 0, y: 0 });
            assert_eq!(region.len(), 4);
            assert!(region.contains(&Position { x: 0, y: 0 }));
            assert!(region.contains(&Position { x: 1, y: 0 }));
            assert!(region.contains(&Position { x: 2, y: 0 }));
            assert!(region.contains(&Position { x: 3, y: 0 }));
        }

        #[test]
        fn test_find_all_regions() {
            let grid = Grid::new(parse_input(
                "\
AAAA
BBCD
BBCC
EEEC",
            ));

            let regions = find_all_regions(&grid);
            assert_eq!(regions.len(), 5); // A, B, C, D, and E regions
        }
    }

    mod calculate_price {
        use super::*;

        #[test]
        fn simple_square() {
            let input = "\
AAAA
AAAA
AAAA
AAAA";
            let (result, _) = calculate_price(input);
            assert_eq!(result, 256);
        }

        #[test]
        fn example_from_prompt() {
            let input = "\
AAAA
BBCD
BBCC
EEEC";
            let (result, _) = calculate_price(input);
            assert_eq!(result, 140);
        }

        #[test]
        fn region_with_holes() {
            let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
            let (result, _) = calculate_price(input);
            assert_eq!(result, 772);
        }
    }
}
