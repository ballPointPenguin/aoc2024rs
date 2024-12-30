use std::fs::read_to_string;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let input = read_to_string("./14-input.txt")?;
    let result = calculate_safety_factor(&input, 101, 103, 100);
    println!("Result: {}", result);

    let end = Instant::now();
    println!("Time taken (Part 1): {:?}", end.duration_since(start));

    let robots = parse_input(&input);
    let result_2 = find_pattern_time(&robots, 101, 103);
    println!("Result (Part 2): {}", result_2);

    let end_2 = Instant::now();
    println!("Time taken (Part 2): {:?}", end_2.duration_since(end));

    println!("{}", render_grid(&robots, 101, 103, result_2 as i32));

    Ok(())
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            // Extract all numbers from the line, including negative signs
            let nums: Vec<i32> = line
                .split(&[',', ' ', '=', 'p', 'v'][..]) // split on any of these chars
                .filter_map(|s| s.parse().ok()) // keep only the parseable numbers
                .collect();

            Robot::new(nums[0], nums[1], nums[2], nums[3])
        })
        .collect()
}

fn render_grid(robots: &[Robot], width: i32, height: i32, seconds: i32) -> String {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for robot in robots {
        let pos = robot.position_at(seconds, width, height);
        grid[pos.y as usize][pos.x as usize] = '🎄';
    }

    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&c| match c {
                    '.' => format!("\x1b[0m."),
                    '🎄' => format!("\x1b[32m🎄"),
                    _ => unreachable!(),
                })
                .collect::<String>()
                + "\x1b[0m"
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn calculate_safety_factor(input: &str, width: i32, height: i32, seconds: i32) -> u64 {
    let robots = parse_input(input);
    let positions: Vec<Position> = robots
        .iter()
        .map(|robot| robot.position_at(seconds, width, height))
        .collect();
    count_quadrants(&positions, width, height).iter().product()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Position,
    v: Velocity,
}

impl Robot {
    fn new(px: i32, py: i32, vx: i32, vy: i32) -> Self {
        Self {
            p: Position { x: px, y: py },
            v: Velocity { x: vx, y: vy },
        }
    }

    fn position_at(&self, seconds: i32, width: i32, height: i32) -> Position {
        // Calculate total movement
        let dx = self.v.x * seconds;
        let dy = self.v.y * seconds;

        // Add to starting pos and convert to positive
        let mut x = (self.p.x + dx) % width;
        if x < 0 {
            x += width;
        }

        let mut y = (self.p.y + dy) % height;
        if y < 0 {
            y += height;
        }

        Position { x, y }
    }
}

fn count_quadrants(positions: &[Position], width: i32, height: i32) -> [u64; 4] {
    let mid_x = width / 2;
    let mid_y = height / 2;

    // Use iterator methods instead of explicit loop
    positions.iter().fold([0; 4], |mut counts, pos| {
        let quadrant = match (pos.x.cmp(&mid_x), pos.y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 2,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3,
            _ => return counts, // Handle points exactly on the lines
        };
        counts[quadrant] += 1;
        counts
    })
}

fn find_pattern_time(robots: &[Robot], width: i32, height: i32) -> i32 {
    // Calculate x-entropy for all positions in width cycle
    let x_entropies: Vec<f64> = (0..width)
        .map(|t| {
            measure_entropy(
                &robots
                    .iter()
                    .map(|r| r.position_at(t, width, height).x)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let best_x_time = best_time(&x_entropies);

    // Calculate y-entropy for all positions in height cycle
    let y_entropies: Vec<f64> = (0..height)
        .map(|t| {
            measure_entropy(
                &robots
                    .iter()
                    .map(|r| r.position_at(t, width, height).y)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let best_y_time = best_time(&y_entropies);

    chinese_reindeer(best_x_time, best_y_time, width, height)
}

fn best_time(entropies: &[f64]) -> i32 {
    entropies
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(t, _)| t as i32)
        .unwrap()
}

fn measure_entropy(positions: &[i32]) -> f64 {
    // Count frequencies of coordinates
    let mut counts = vec![0; 256];

    for &pos in positions {
        counts[pos as usize] += 1;
    }

    // Calculate entropy - lower means more orderly
    counts
        .iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / positions.len() as f64;
            -p * p.ln()
        })
        .sum()
}

// Chinese Remainder Theorem
fn chinese_reindeer(a1: i32, a2: i32, n1: i32, n2: i32) -> i32 {
    // Extended Euclidean Algorithm to find Bézout's coefficients
    fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (gcd, x1, y1) = extended_gcd(b, a % b);
            (gcd, y1, x1 - (a / b) * y1)
        }
    }

    // Find modular multiplicative inverse
    let (_, m1, m2) = extended_gcd(n1, n2);

    // Calculate result using Chinese Remainder Theorem formula
    let mut result = a1 * m2 * n2 + a2 * m1 * n1;
    let n = n1 * n2;

    // Normalize result to smallest positive number
    result = ((result % n) + n) % n;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    mod position_at {
        use super::*;

        #[test]
        fn at_0_seconds() {
            let robot = Robot::new(2, 4, 2, -3);
            assert_eq!(robot.position_at(0, 11, 7), Position { x: 2, y: 4 });
        }

        #[test]
        fn at_1_second() {
            let robot = Robot::new(2, 4, 2, -3);
            assert_eq!(robot.position_at(1, 11, 7), Position { x: 4, y: 1 });
        }

        #[test]
        fn at_2_seconds() {
            let robot = Robot::new(2, 4, 2, -3);
            assert_eq!(robot.position_at(2, 11, 7), Position { x: 6, y: 5 });
        }

        #[test]
        fn at_3_seconds() {
            let robot = Robot::new(2, 4, 2, -3);
            assert_eq!(robot.position_at(3, 11, 7), Position { x: 8, y: 2 });
        }

        #[test]
        fn at_4_seconds() {
            let robot = Robot::new(2, 4, 2, -3);
            assert_eq!(robot.position_at(4, 11, 7), Position { x: 10, y: 6 });
        }

        #[test]
        fn at_5_seconds() {
            let robot = Robot::new(2, 4, 2, -3);
            assert_eq!(robot.position_at(5, 11, 7), Position { x: 1, y: 3 });
        }
    }

    #[test]
    fn test_parse_input() {
        let robots = parse_input(EXAMPLE_INPUT);
        assert_eq!(robots.len(), 12);
    }

    #[test]
    fn test_count_quadrants() {
        // Create a set of final positions that matches the example's final state
        let positions = vec![
            Position { x: 6, y: 0 },
            Position { x: 6, y: 0 },
            Position { x: 9, y: 0 },
            Position { x: 0, y: 2 },
            Position { x: 1, y: 3 },
            Position { x: 2, y: 3 },
            Position { x: 5, y: 4 },
            Position { x: 3, y: 5 },
            Position { x: 4, y: 5 },
            Position { x: 4, y: 5 },
            Position { x: 1, y: 6 },
            Position { x: 6, y: 6 },
        ];
        assert_eq!(count_quadrants(&positions, 11, 7), [1, 3, 4, 1]);
    }

    #[test]
    fn test_calculate_safety_factor() {
        assert_eq!(calculate_safety_factor(EXAMPLE_INPUT, 11, 7, 100), 12);
    }
}
