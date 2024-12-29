use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
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
    cells: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
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
    direction: Direction,
    edge_type: EdgeType,
    is_inner: bool,
}

impl Grid {
    fn new(cells: Vec<Vec<u8>>) -> Self {
        Self {
            cells: cells.clone(),
            width: cells[0].len(),
            height: cells.len(),
        }
    }

    fn get_cell(&self, pos: Position) -> u8 {
        self.cells[pos.y][pos.x]
    }

    fn calculate_perimeter(&self, region: &HashSet<Position>) -> usize {
        let mut perimeter = 0;
        for &pos in region {
            let neighbors = get_neighbors(self, pos);

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

    fn calculate_sides(&self, region: &HashSet<Position>) -> usize {
        let edges = self.collect_edges(region);
        // Count unique sides
        // Edges with same direction and adjacent positions should count as 1 side

        // Group edges by edge_type and inner/outer
        let mut top_inner = Vec::new();
        let mut top_outer = Vec::new();
        let mut bottom_inner = Vec::new();
        let mut bottom_outer = Vec::new();
        let mut left_inner = Vec::new();
        let mut left_outer = Vec::new();
        let mut right_inner = Vec::new();
        let mut right_outer = Vec::new();

        for edge in edges {
            match (edge.edge_type, edge.is_inner) {
                (EdgeType::Top, true) => top_inner.push(edge),
                (EdgeType::Top, false) => top_outer.push(edge),
                (EdgeType::Bottom, true) => bottom_inner.push(edge),
                (EdgeType::Bottom, false) => bottom_outer.push(edge),
                (EdgeType::Left, true) => left_inner.push(edge),
                (EdgeType::Left, false) => left_outer.push(edge),
                (EdgeType::Right, true) => right_inner.push(edge),
                (EdgeType::Right, false) => right_outer.push(edge),
            }
        }

        // Helper function to count sides in a group of edges
        fn count_sides(edges: &[Edge]) -> usize {
            if edges.is_empty() {
                return 0;
            }

            // Group by row/column first
            let mut by_line: HashMap<usize, Vec<Edge>> = HashMap::new();
            for edge in edges {
                let key = match edge.edge_type {
                    EdgeType::Top | EdgeType::Bottom => edge.start.y,
                    EdgeType::Left | EdgeType::Right => edge.start.x,
                };
                by_line.entry(key).or_default().push(*edge);
            }

            // For each line, count continuous segments
            by_line
                .values()
                .map(|line_edges| {
                    let mut sorted_edges = line_edges.to_vec();
                    sorted_edges.sort_by_key(|e| match e.edge_type {
                        EdgeType::Top | EdgeType::Bottom => e.start.x,
                        EdgeType::Left | EdgeType::Right => e.start.y,
                    });

                    let mut segments = 1;
                    for window in sorted_edges.windows(2) {
                        let pos1 = match window[0].edge_type {
                            EdgeType::Top | EdgeType::Bottom => window[0].start.x,
                            EdgeType::Left | EdgeType::Right => window[0].start.y,
                        };
                        let pos2 = match window[1].edge_type {
                            EdgeType::Top | EdgeType::Bottom => window[1].start.x,
                            EdgeType::Left | EdgeType::Right => window[1].start.y,
                        };

                        // If positiions aren't adjacent or y-coords differ (for vertical)
                        // or x-coords differ (for horizontal), count as new segment
                        if pos1.abs_diff(pos2) > 1
                            || (matches!(window[0].edge_type, EdgeType::Left | EdgeType::Right)
                                && window[0].start.x != window[1].start.x)
                            || (matches!(window[0].edge_type, EdgeType::Top | EdgeType::Bottom)
                                && window[0].start.y != window[1].start.y)
                        {
                            segments += 1;
                        }
                    }
                    segments
                })
                .sum()
        }

        let ti_sides = count_sides(&top_inner);
        let to_sides = count_sides(&top_outer);
        let bi_sides = count_sides(&bottom_inner);
        let bo_sides = count_sides(&bottom_outer);
        let li_sides = count_sides(&left_inner);
        let lo_sides = count_sides(&left_outer);
        let ri_sides = count_sides(&right_inner);
        let ro_sides = count_sides(&right_outer);

        ti_sides + to_sides + bi_sides + bo_sides + li_sides + lo_sides + ri_sides + ro_sides
    }

    fn collect_edges(&self, region: &HashSet<Position>) -> HashSet<Edge> {
        // For each position in the region:
        // - Check its neighbors
        // - For each missing neighbor (boundary):
        //   - create Edge with appropriate direction
        let mut edges = HashSet::new();

        for &pos in region {
            let neighbors = get_neighbors(self, pos);

            // For each possible direction:
            // Top edge
            if !neighbors.iter().any(|n| n.y < pos.y && region.contains(n)) {
                edges.insert(Edge {
                    start: pos,
                    direction: Direction::Horizontal,
                    edge_type: EdgeType::Top,
                    is_inner: neighbors.iter().any(|n| n.y < pos.y),
                });
            }
            // Bottom edge
            if !neighbors.iter().any(|n| n.y > pos.y && region.contains(n)) {
                edges.insert(Edge {
                    start: pos,
                    direction: Direction::Horizontal,
                    edge_type: EdgeType::Bottom,
                    is_inner: neighbors.iter().any(|n| n.y > pos.y),
                });
            }
            // Left edge
            if !neighbors.iter().any(|n| n.x < pos.x && region.contains(n)) {
                edges.insert(Edge {
                    start: pos,
                    direction: Direction::Vertical,
                    edge_type: EdgeType::Left,
                    is_inner: neighbors.iter().any(|n| n.x < pos.x),
                });
            }
            // Right edge
            if !neighbors.iter().any(|n| n.x > pos.x && region.contains(n)) {
                edges.insert(Edge {
                    start: pos,
                    direction: Direction::Vertical,
                    edge_type: EdgeType::Right,
                    is_inner: neighbors.iter().any(|n| n.x > pos.x),
                });
            }
        }
        edges
    }
}

fn find_region(grid: &Grid, start: Position) -> HashSet<Position> {
    let find_region_start = Instant::now();
    // Do a flood fill to find all connected (horizontal or vertical) cells of the same type
    let mut region = HashSet::new();
    let mut visited = vec![vec![false; grid.width]; grid.height];
    let mut queue = VecDeque::new();

    let value = grid.get_cell(start);
    queue.push_back(start);
    visited[start.y][start.x] = true;
    region.insert(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in get_neighbors(grid, current) {
            if !visited[neighbor.y][neighbor.x] && grid.get_cell(neighbor) == value {
                visited[neighbor.y][neighbor.x] = true;
                region.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    let find_region_end = Instant::now();
    let duration = find_region_end.duration_since(find_region_start);
    if duration.as_secs() > 1 {
        println!(
            "Time taken to find region for {}: {:?}",
            value as char, duration
        );
    }

    region
}

fn get_neighbors(grid: &Grid, pos: Position) -> Vec<Position> {
    let mut neighbors = Vec::new();

    // Check all 4 cardinal directions
    let deltas = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for (dx, dy) in deltas {
        let new_x = pos.x as i32 + dx;
        let new_y = pos.y as i32 + dy;
        if new_x >= 0 && new_x < grid.width as i32 && new_y >= 0 && new_y < grid.height as i32 {
            neighbors.push(Position {
                x: new_x as usize,
                y: new_y as usize,
            });
        }
    }
    neighbors
}

fn find_all_regions(grid: &Grid) -> Vec<HashSet<Position>> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = Position { x, y };
            if visited.contains(&pos) {
                continue;
            }
            let region = find_region(grid, pos);
            regions.push(region.clone());
            visited.extend(region);
        }
    }
    regions
}

#[cfg(test)]
mod tests {
    use super::*;

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
        }

        #[test]
        fn region_with_holes() {
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
