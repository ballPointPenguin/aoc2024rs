use rayon::prelude::*;
use std::fs::read_to_string;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let input = read_to_string("./13-input.txt")?;
    let machines = parse_input(&input);
    let result = calculate_tokens(&machines);

    println!("Result: {}", result);

    let end = Instant::now();
    println!("Part 1 Time taken: {:?}", end.duration_since(start));

    let big_machines = augment_machines(&machines);
    let result_2 = calculate_tokens(&big_machines);

    println!("Result 2: {}", result_2);

    let end_2 = Instant::now();
    println!("Part 2 Time taken: {:?}", end_2.duration_since(end));

    Ok(())
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine| {
            let lines = machine.lines().collect::<Vec<&str>>();
            let button_a = lines[0].split("X+").collect::<Vec<&str>>()[1]
                .split(", Y+")
                .collect::<Vec<&str>>();
            let button_b = lines[1].split("X+").collect::<Vec<&str>>()[1]
                .split(", Y+")
                .collect::<Vec<&str>>();
            let prize = lines[2].split("X=").collect::<Vec<&str>>()[1]
                .split(", Y=")
                .collect::<Vec<&str>>();

            Machine {
                button_a: (button_a[0].parse().unwrap(), button_a[1].parse().unwrap()),
                button_b: (button_b[0].parse().unwrap(), button_b[1].parse().unwrap()),
                prize: (prize[0].parse().unwrap(), prize[1].parse().unwrap()),
            }
        })
        .collect()
}

fn calculate_tokens(machines: &Vec<Machine>) -> i64 {
    machines
        .par_iter()
        .map(|machine| {
            let (a, b) = machine.solve();
            3 * a + b
        })
        .sum()
}

fn augment_machines(machines: &Vec<Machine>) -> Vec<Machine> {
    machines
        .iter()
        .map(|machine| Machine {
            prize: (
                machine.prize.0 + 10_000_000_000_000,
                machine.prize.1 + 10_000_000_000_000,
            ),
            ..*machine
        })
        .collect()
}

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn solve(&self) -> (i64, i64) {
        let (x, y) = self.prize;
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;

        // Calculate determinant of the coefficient matrix
        let det = ay * bx - ax * by;
        if det == 0 {
            return (0, 0);
        }

        // Calculate a using derived formula
        let numerator = y * bx - x * by;
        let a = numerator as f64 / det as f64;

        // Check if a is a positive integer
        if a.fract() != 0.0 || a < 0.0 {
            return (0, 0);
        }

        let a = a as i64;

        // Now try and solve to get b
        if let Some(b) = self.try_solve_with_a(a) {
            (a, b)
        } else {
            (0, 0)
        }
    }

    fn try_solve_with_a(&self, a: i64) -> Option<i64> {
        let (x, y) = self.prize;
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;

        let b = (x - a * ax) / bx;
        if b < 0 {
            return None;
        }

        if a * ay + b * by != y {
            return None;
        }

        Some(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MACHINE_1: Machine = Machine {
        button_a: (94, 34),
        button_b: (22, 67),
        prize: (8400, 5400),
    };

    const MACHINE_2: Machine = Machine {
        button_a: (26, 66),
        button_b: (67, 21),
        prize: (12748, 12176),
    };

    const MACHINE_3: Machine = Machine {
        button_a: (17, 86),
        button_b: (84, 37),
        prize: (7870, 6450),
    };

    const MACHINE_4: Machine = Machine {
        button_a: (69, 23),
        button_b: (27, 71),
        prize: (18641, 10279),
    };

    mod solve_machine {
        use super::*;

        #[test]
        fn simple_machine() {
            let machine = Machine {
                button_a: (1, 2),
                button_b: (3, 1),
                prize: (7, 4),
            };
            assert_eq!(machine.solve(), (1, 2));
        }

        #[test]
        fn example_machines() {
            assert_eq!(MACHINE_1.solve(), (80, 40));
            assert_eq!(MACHINE_3.solve(), (38, 86));
        }

        #[test]
        fn unsolvable_machines() {
            assert_eq!(MACHINE_2.solve(), (0, 0));
            assert_eq!(MACHINE_4.solve(), (0, 0));
        }
    }

    mod calculate_tokens {
        use super::*;

        #[test]
        fn example_machines() {
            let machines = vec![MACHINE_1, MACHINE_2, MACHINE_3, MACHINE_4];
            assert_eq!(calculate_tokens(&machines), 480);
        }
    }
}
