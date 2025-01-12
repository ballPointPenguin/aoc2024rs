use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn shifted(self, dx: isize, dy: isize) -> Option<Position> {
        let new_x = self.x.checked_add_signed(dx)?;
        let new_y = self.y.checked_add_signed(dy)?;
        Some(Position { x: new_x, y: new_y })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Box {
    left: Position,
    right: Position,
}

impl Box {
    fn new(left: Position, right: Position) -> Self {
        assert_eq!(left.y, right.y, "Box positions must be on same row");
        assert_eq!(
            left.x + 1,
            right.x,
            "Box positions must be horizontally adjacent"
        );
        Self { left, right }
    }

    fn gps_coord(&self) -> usize {
        self.left.x + self.left.y * 100
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::BoxLeft => '[',
                Tile::BoxRight => ']',
                Tile::Robot => '@',
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn deltas(&self) -> (isize, isize) {
        match self {
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Up => (0, -1),
            Move::Down => (0, 1),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Warehouse {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
    boxes: HashSet<Box>,
    robot: Position,
}

impl Warehouse {
    fn new(grid: &Vec<Vec<Tile>>) -> Self {
        let mut warehouse = Self {
            width: grid[0].len(),
            height: grid.len(),
            walls: HashSet::new(),
            boxes: HashSet::new(),
            robot: Position::default(),
        };

        for (r, row) in grid.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Wall => {
                        warehouse.walls.insert(Position { x: c, y: r });
                    }
                    Tile::BoxLeft => {
                        warehouse.boxes.insert(Box::new(
                            Position { x: c, y: r },
                            Position { x: c + 1, y: r },
                        ));
                    }
                    Tile::Robot => {
                        warehouse.robot = Position { x: c, y: r };
                    }
                    _ => {}
                }
            }
        }

        warehouse
    }

    fn grid(&self) -> Vec<Vec<Tile>> {
        let mut grid = vec![vec![Tile::Empty; self.width]; self.height];

        // Add walls
        for wall in &self.walls {
            grid[wall.y][wall.x] = Tile::Wall;
        }

        // Add boxes (both left and right sides)
        for bx in &self.boxes {
            grid[bx.left.y][bx.left.x] = Tile::BoxLeft;
            grid[bx.right.y][bx.right.x] = Tile::BoxRight;
        }

        // Add robot
        grid[self.robot.y][self.robot.x] = Tile::Robot;

        grid
    }

    fn display(&self) -> String {
        self.grid()
            .iter()
            .map(|row| row.iter().map(|t| t.to_string()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn sum_boxes_gps_coord(&self) -> usize {
        self.boxes.iter().map(|b| b.gps_coord()).sum()
    }

    fn get_tile(&self, pos: &Position) -> Tile {
        if self.walls.contains(pos) {
            Tile::Wall
        } else if self.boxes.iter().any(|b| b.left == *pos) {
            Tile::BoxLeft
        } else if self.boxes.iter().any(|b| b.right == *pos) {
            Tile::BoxRight
        } else if self.robot == *pos {
            Tile::Robot
        } else {
            Tile::Empty
        }
    }

    fn is_in_bounds(&self, pos: &Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    fn find_box_at_pos(&self, pos: &Position) -> Option<Box> {
        self.boxes
            .iter()
            .find(|bx| bx.left == *pos || bx.right == *pos)
            .copied()
    }

    fn make_moves(&mut self, moves: &[Move]) {
        for &dir in moves {
            self.try_move(dir);
        }
    }

    fn try_move(&mut self, dir: Move) {
        let (dx, dy) = dir.deltas();
        let Some(next_pos) = self.robot.shifted(dx, dy) else {
            return;
        };

        let visited = if dy != 0 {
            // For vertical movement, use explore_vertical_push
            match self.explore_vertical_push(next_pos, dy) {
                None => return,
                Some(boxes) => boxes,
            }
        } else {
            // For horizontal movement, scan linearly
            let boxes = self.explore_horizontal_push(next_pos, dx);
            if boxes.is_empty() && !matches!(self.get_tile(&next_pos), Tile::Empty) {
                return;
            }
            boxes
        };

        // If we get here, movement is possible
        let sorted_boxes = self.sort_boxes(&visited, dir);
        for bx in sorted_boxes {
            self.move_box(bx, dx, dy);
        }
        self.robot = next_pos;
    }

    fn explore_vertical_push(&self, start: Position, dy: isize) -> Option<HashSet<Box>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut blocked = false;

        queue.push_back(start);

        while let Some(pos) = queue.pop_front() {
            if !self.is_in_bounds(&pos) {
                blocked = true;
                continue;
            }

            match self.get_tile(&pos) {
                Tile::Wall => {
                    blocked = true;
                }
                Tile::BoxLeft | Tile::BoxRight => {
                    let bx = self.find_box_at_pos(&pos).unwrap();

                    if visited.insert(bx) {
                        let next_left = Position {
                            x: bx.left.x,
                            y: (bx.left.y as isize + dy) as usize,
                        };
                        let next_right = Position {
                            x: bx.right.x,
                            y: (bx.right.y as isize + dy) as usize,
                        };

                        queue.push_back(next_left);
                        queue.push_back(next_right);
                    }
                }
                Tile::Empty => {}
                Tile::Robot => panic!("Robot found at {:?}", pos),
            }
        }

        if blocked {
            None
        } else {
            Some(visited)
        }
    }

    fn explore_horizontal_push(&self, start: Position, dx: isize) -> HashSet<Box> {
        let mut visited = HashSet::new();
        let mut scan_pos = start;

        // First check if we can move at all
        if !matches!(self.get_tile(&scan_pos), Tile::BoxLeft | Tile::BoxRight) {
            return visited;
        }

        // Then scan for boxes until we hit a wall or empty space
        loop {
            if !self.is_in_bounds(&scan_pos) {
                return HashSet::new();
            }

            match self.get_tile(&scan_pos) {
                Tile::Wall => return HashSet::new(),
                Tile::Empty => break,
                Tile::BoxLeft | Tile::BoxRight => {
                    let Some(bx) = self.find_box_at_pos(&scan_pos) else {
                        return HashSet::new();
                    };

                    visited.insert(bx);

                    let Some(next_scan) = scan_pos.shifted(dx, 0) else {
                        return HashSet::new();
                    };
                    scan_pos = next_scan;
                }
                Tile::Robot => panic!("Robot found at {:?}", scan_pos),
            }
        }

        visited
    }

    fn sort_boxes(&self, boxes: &HashSet<Box>, dir: Move) -> Vec<Box> {
        let mut sorted_boxes = boxes.iter().cloned().collect::<Vec<_>>();
        let ascending = matches!(dir, Move::Up | Move::Left);
        sorted_boxes.sort_by(|a, b| {
            let key_a = match dir {
                Move::Up | Move::Down => a.left.y,
                Move::Left | Move::Right => a.left.x,
            };
            let key_b = match dir {
                Move::Up | Move::Down => b.left.y,
                Move::Left | Move::Right => b.left.x,
            };
            if ascending {
                key_a.cmp(&key_b)
            } else {
                key_b.cmp(&key_a)
            }
        });
        sorted_boxes
    }

    fn move_box(&mut self, bx: Box, dx: isize, dy: isize) {
        self.boxes.remove(&bx);

        let new_left = Position {
            x: (bx.left.x as isize + dx) as usize,
            y: (bx.left.y as isize + dy) as usize,
        };
        let new_right = Position {
            x: (bx.right.x as isize + dx) as usize,
            y: (bx.right.y as isize + dy) as usize,
        };
        let new_box = Box::new(new_left, new_right);
        self.boxes.insert(new_box);
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidMapChar(char),
    InvalidMoveChar(char),
    EmptyInput,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidMapChar(c) => write!(f, "Invalid character in map: {}", c),
            ParseError::InvalidMoveChar(c) => write!(f, "Invalid move character: {}", c),
            ParseError::EmptyInput => write!(f, "Input is empty"),
        }
    }
}

impl std::error::Error for ParseError {}

fn parse_input(input: &str) -> Result<(Warehouse, Vec<Move>), ParseError> {
    let mut lines = input.lines();

    // Parse grid
    let grid: Vec<Vec<Tile>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Ok(vec![Tile::Wall, Tile::Wall]),
                    'O' => Ok(vec![Tile::BoxLeft, Tile::BoxRight]),
                    '@' => Ok(vec![Tile::Robot, Tile::Empty]),
                    '.' => Ok(vec![Tile::Empty, Tile::Empty]),
                    c => Err(ParseError::InvalidMapChar(c)),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|v| v.into_iter().flatten().collect())
        })
        .collect::<Result<Vec<_>, _>>()?;

    if grid.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // Parse moves
    let moves: Vec<Move> = lines
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            c => Err(ParseError::InvalidMoveChar(c)),
        })
        .collect::<Result<_, _>>()?;

    Ok((Warehouse::new(&grid), moves))
}

fn main() {
    let input = fs::read_to_string("15-input.txt").expect("Unable to read file");
    let (mut warehouse, moves) = parse_input(&input).expect("Failed to parse input");

    println!("INITIAL STATE:\n{}", warehouse.display());

    warehouse.make_moves(&moves);
    let result = warehouse.sum_boxes_gps_coord();

    println!("FINAL STATE:\n{}", warehouse.display());
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const LARGE_EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const FINAL_GRID: &str = "\
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";

    mod parse_input {
        use super::*;

        #[test]
        fn parse_moves() {
            let (_warehouse, moves) = parse_input(SMALL_EXAMPLE).unwrap();
            assert_eq!(
                moves,
                vec![
                    Move::Left,
                    Move::Down,
                    Move::Down,
                    Move::Left,
                    Move::Left,
                    Move::Up,
                    Move::Up,
                    Move::Left,
                    Move::Left,
                    Move::Up,
                    Move::Up,
                ]
            );
        }

        #[test]
        fn small_example() {
            const EXPECTED: &str = "\
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############";

            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }

        #[test]
        fn large_example() {
            const EXPECTED: &str = "\
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";

            let (warehouse, _moves) = parse_input(LARGE_EXAMPLE).unwrap();
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }
    }

    mod warehouse {
        use super::*;

        #[test]
        fn place_robot() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            assert_eq!(warehouse.robot, Position { x: 10, y: 3 });
        }

        #[test]
        fn place_walls() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            assert!(warehouse.walls.is_superset(&HashSet::from([
                Position { x: 8, y: 1 },
                Position { x: 9, y: 1 }
            ])));
        }

        #[test]
        fn place_boxes() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            assert_eq!(
                warehouse.boxes,
                HashSet::from([
                    Box::new(Position { x: 6, y: 3 }, Position { x: 7, y: 3 }),
                    Box::new(Position { x: 8, y: 3 }, Position { x: 9, y: 3 }),
                    Box::new(Position { x: 6, y: 4 }, Position { x: 7, y: 4 }),
                ])
            );
        }

        #[test]
        fn warehouse_to_grid() {
            let input = vec![
                vec![Tile::Wall, Tile::Wall, Tile::Wall],
                vec![Tile::Wall, Tile::BoxLeft, Tile::Robot],
                vec![Tile::Wall, Tile::Wall, Tile::Wall],
            ];
            let warehouse = Warehouse::new(&input);
            let output = warehouse.grid();
            assert_eq!(input, output);
        }
    }

    mod sorted_boxes {
        use super::*;

        #[test]
        fn vertical_stack_down() {
            let boxes = HashSet::from([
                Box::new(Position { x: 10, y: 4 }, Position { x: 11, y: 4 }),
                Box::new(Position { x: 10, y: 3 }, Position { x: 11, y: 3 }),
                Box::new(Position { x: 10, y: 5 }, Position { x: 11, y: 5 }),
            ]);
            let warehouse = Warehouse {
                width: 20,
                height: 20,
                walls: HashSet::new(),
                boxes: HashSet::new(),
                robot: Position { x: 0, y: 0 },
            };
            let sorted = warehouse.sort_boxes(&boxes, Move::Down);
            assert_eq!(
                sorted,
                vec![
                    Box::new(Position { x: 10, y: 5 }, Position { x: 11, y: 5 }),
                    Box::new(Position { x: 10, y: 4 }, Position { x: 11, y: 4 }),
                    Box::new(Position { x: 10, y: 3 }, Position { x: 11, y: 3 }),
                ]
            );
        }

        #[test]
        fn vertical_stack_up() {
            let boxes = HashSet::from([
                Box::new(Position { x: 10, y: 4 }, Position { x: 11, y: 4 }),
                Box::new(Position { x: 10, y: 3 }, Position { x: 11, y: 3 }),
                Box::new(Position { x: 10, y: 5 }, Position { x: 11, y: 5 }),
            ]);
            let warehouse = Warehouse {
                width: 20,
                height: 20,
                walls: HashSet::new(),
                boxes: HashSet::new(),
                robot: Position { x: 0, y: 0 },
            };
            let sorted = warehouse.sort_boxes(&boxes, Move::Up);
            assert_eq!(
                sorted,
                vec![
                    Box::new(Position { x: 10, y: 3 }, Position { x: 11, y: 3 }),
                    Box::new(Position { x: 10, y: 4 }, Position { x: 11, y: 4 }),
                    Box::new(Position { x: 10, y: 5 }, Position { x: 11, y: 5 }),
                ]
            );
        }
    }

    mod try_move {
        use super::*;

        #[test]
        fn move_left() {
            const EXPECTED: &str = "\
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############";

            let (mut warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            warehouse.try_move(Move::Left);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }

        #[test]
        fn move_left_accumulate() {
            const INPUT: &str = "\
##########
#.O.OO.O@#
##########

";

            const EXPECTED: &str = "\
####################
##[][][][]@.......##
####################";

            let (mut warehouse, _moves) = parse_input(INPUT).unwrap();
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }

        #[test]
        fn multi_move() {
            const EXPECTED: &str = "\
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############";

            let (mut warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Down);
            warehouse.try_move(Move::Down);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Left);
            warehouse.try_move(Move::Up);
            warehouse.try_move(Move::Up);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }
    }

    mod make_moves {
        use super::*;

        #[test]
        fn small_example() {
            const EXPECTED: &str = "\
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############";

            let (mut warehouse, moves) = parse_input(SMALL_EXAMPLE).unwrap();
            warehouse.make_moves(&moves);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
            assert_eq!(warehouse.sum_boxes_gps_coord(), 105 + 207 + 306);
        }

        #[test]
        fn large_example() {
            let (mut warehouse, moves) = parse_input(LARGE_EXAMPLE).unwrap();
            warehouse.make_moves(&moves);
            let display = warehouse.display();
            assert_eq!(display, FINAL_GRID);
            assert_eq!(warehouse.sum_boxes_gps_coord(), 9021);
        }
    }

    mod gps_coords {
        use super::*;

        #[test]
        fn single_box() {
            let box1 = Box::new(Position { x: 5, y: 1 }, Position { x: 6, y: 1 });
            assert_eq!(box1.gps_coord(), 105);
        }

        #[test]
        fn small_grid() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE).unwrap();
            assert_eq!(warehouse.sum_boxes_gps_coord(), 306 + 308 + 406);
        }
    }
}
