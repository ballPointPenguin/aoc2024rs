use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        Self { left, right }
    }

    fn gps_coord(&self) -> usize {
        self.left.x + self.left.y * 100
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
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
            robot: Position { x: 0, y: 0 },
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
}

fn make_moves(warehouse: &mut Warehouse, moves: &Vec<Move>) {
    for dir in moves {
        try_move(warehouse, *dir);
    }
}

fn try_move(warehouse: &mut Warehouse, dir: Move) {
    match dir {
        Move::Up | Move::Down => try_move_vert(warehouse, dir),
        Move::Left | Move::Right => try_move_horiz(warehouse, dir),
    }
}

fn try_move_vert(warehouse: &mut Warehouse, dir: Move) {
    let (dx, dy) = deltas(&dir);

    let Some(next_pos) = warehouse.robot.shifted(dx, dy) else {
        return;
    };

    let result = explore_vertical_push(warehouse, next_pos, dy);
    match result {
        None => {} // Blocked. Do nothing
        Some(visited) => {
            let sorted_boxes = sorted_boxes(&visited, dir);
            for bx in sorted_boxes {
                move_box(warehouse, bx, dx, dy);
            }
            warehouse.robot = next_pos;
        }
    }
}

fn sorted_boxes(boxes: &HashSet<Box>, dir: Move) -> Vec<Box> {
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

fn explore_vertical_push(
    warehouse: &Warehouse,
    start: Position,
    dy: isize,
) -> Option<HashSet<Box>> {
    // DFS to find all boxes that can be pushed
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);

    let mut found_empty = false;
    let mut blocked = false;

    while let Some(pos) = queue.pop_front() {
        if !warehouse.is_in_bounds(&pos) {
            blocked = true;
            continue;
        }

        match warehouse.get_tile(&pos) {
            Tile::Wall => blocked = true,
            Tile::Empty => found_empty = true,
            Tile::BoxLeft | Tile::BoxRight => {
                let Some(bx) = warehouse.find_box_at_pos(&pos) else {
                    // Should never happen
                    blocked = true;
                    continue;
                };

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
            // Should never happen
            _ => panic!("Unexpected tile"),
        }
    }

    if blocked {
        None
    } else if found_empty {
        Some(visited)
    } else {
        // Should never happen
        None
    }
}

fn try_move_horiz(warehouse: &mut Warehouse, dir: Move) {
    let (dx, dy) = deltas(&dir);

    let Some(next_pos) = warehouse.robot.shifted(dx, dy) else {
        return;
    };

    let mut visited = HashSet::new();

    let mut scan_pos = next_pos;

    loop {
        if !warehouse.is_in_bounds(&scan_pos) {
            return;
        }

        match warehouse.get_tile(&scan_pos) {
            Tile::Wall => return,
            Tile::Empty => break,
            Tile::BoxLeft | Tile::BoxRight => {
                let Some(bx) = warehouse.find_box_at_pos(&scan_pos) else {
                    return;
                };

                visited.insert(bx);

                let Some(next_scan) = scan_pos.shifted(dx, dy) else {
                    return;
                };
                scan_pos = next_scan;
                continue;
            }
            // Should never happen
            Tile::Robot => panic!("Robot found at {:?}", scan_pos),
        }
    }

    // If we got here, we found an empty tile (break)
    let sorted_boxes = sorted_boxes(&visited, dir);
    for bx in sorted_boxes {
        move_box(warehouse, bx, dx, dy);
    }
    warehouse.robot = next_pos;
}

fn move_box(warehouse: &mut Warehouse, bx: Box, dx: isize, dy: isize) {
    warehouse.boxes.remove(&bx);

    let new_left = Position {
        x: (bx.left.x as isize + dx) as usize,
        y: (bx.left.y as isize + dy) as usize,
    };
    let new_right = Position {
        x: (bx.right.x as isize + dx) as usize,
        y: (bx.right.y as isize + dy) as usize,
    };
    let new_box = Box::new(new_left, new_right);
    warehouse.boxes.insert(new_box);
}

fn deltas(dir: &Move) -> (isize, isize) {
    match dir {
        Move::Left => (-1, 0),
        Move::Right => (1, 0),
        Move::Up => (0, -1),
        Move::Down => (0, 1),
    }
}

fn parse_input(input: &str) -> (Warehouse, Vec<Move>) {
    let (grid, moves) = split_input_into_grid_and_moves(input);
    (Warehouse::new(&grid), moves)
}

fn split_input_into_grid_and_moves(input: &str) -> (Vec<Vec<Tile>>, Vec<Move>) {
    let mut lines = input.lines();
    let grid: Vec<Vec<Tile>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| parse_tile(c))
                .flat_map(|(t1, t2)| vec![t1, t2])
                .collect()
        })
        .collect();
    let moves: Vec<Move> = lines
        .flat_map(|line| line.chars().map(parse_move))
        .collect();
    (grid, moves)
}

fn parse_tile(c: char) -> (Tile, Tile) {
    match c {
        '#' => (Tile::Wall, Tile::Wall),
        'O' => (Tile::BoxLeft, Tile::BoxRight),
        '@' => (Tile::Robot, Tile::Empty),
        '.' => (Tile::Empty, Tile::Empty),
        _ => panic!("Unexpected character in map: {}", c),
    }
}

fn parse_move(c: char) -> Move {
    match c {
        '^' => Move::Up,
        'v' => Move::Down,
        '<' => Move::Left,
        '>' => Move::Right,
        _ => panic!("Unexpected move character: {}", c),
    }
}

fn main() {
    let input = fs::read_to_string("15-input.txt").expect("Unable to read file");
    let (mut warehouse, moves) = parse_input(&input);

    println!("INITIAL STATE:\n{}", warehouse.display());

    make_moves(&mut warehouse, &moves);
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
        fn small_example() {
            const EXPECTED: &str = "\
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############";

            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE);
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

            let (warehouse, _moves) = parse_input(LARGE_EXAMPLE);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }

        #[test]
        fn parse_moves() {
            let (_warehouse, moves) = parse_input(SMALL_EXAMPLE);
            // <vv<<^^<<^^"
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
    }

    mod warehouse {
        use super::*;

        #[test]
        fn place_robot() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE);
            assert_eq!(warehouse.robot, Position { x: 10, y: 3 });
        }

        #[test]
        fn place_walls() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE);
            assert!(warehouse.walls.is_superset(&HashSet::from([
                Position { x: 8, y: 1 },
                Position { x: 9, y: 1 }
            ])));
        }

        #[test]
        fn place_boxes() {
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE);
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
            let sorted = sorted_boxes(&boxes, Move::Down);
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
            let sorted = sorted_boxes(&boxes, Move::Up);
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

            let (mut warehouse, _moves) = parse_input(SMALL_EXAMPLE);
            try_move(&mut warehouse, Move::Left);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
        }

        #[test]
        fn move_left_accumulate() {
            const INPUT: &str = "\
##########
#.O.OO.O@#
##########";

            const EXPECTED: &str = "\
####################
##[][][][]@.......##
####################";

            let (mut warehouse, _moves) = parse_input(INPUT);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
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

            let (mut warehouse, _moves) = parse_input(SMALL_EXAMPLE);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Down);
            try_move(&mut warehouse, Move::Down);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Left);
            try_move(&mut warehouse, Move::Up);
            try_move(&mut warehouse, Move::Up);
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

            let (mut warehouse, moves) = parse_input(SMALL_EXAMPLE);
            make_moves(&mut warehouse, &moves);
            let display = warehouse.display();
            assert_eq!(display, EXPECTED);
            assert_eq!(warehouse.sum_boxes_gps_coord(), 105 + 207 + 306);
        }

        #[test]
        fn large_example() {
            let (mut warehouse, moves) = parse_input(LARGE_EXAMPLE);
            make_moves(&mut warehouse, &moves);
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
            let (warehouse, _moves) = parse_input(SMALL_EXAMPLE);
            assert_eq!(warehouse.sum_boxes_gps_coord(), 306 + 308 + 406);
        }
    }
}
