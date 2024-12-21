use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./07-input.txt")?;

    let result = total_calibration_result(&input);

    println!("Result: {}", result);

    Ok(())
}

pub fn total_calibration_result(input: &str) -> i64 {
    input
        .lines()
        .map(|line| parse_equation(line))
        .filter(|eq| is_solvable(eq))
        .map(|eq| eq.target)
        .sum()
}

fn parse_equation(input: &str) -> Equation {
    let (target, numbers) = input.split_once(':').unwrap();
    let target = target.parse().unwrap();
    let numbers = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Equation { target, numbers }
}

fn is_solvable(equation: &Equation) -> bool {
    let num_operators = equation.numbers.len() - 1;
    let combinations = 2_i32.pow(num_operators as u32);

    for i in 0..combinations {
        let mut result = equation.numbers[0];

        for pos in 0..num_operators {
            let next_num = equation.numbers[pos + 1];
            // Use bit at position 'pos' to determine operator
            let use_multiply = (i & (1 << pos)) != 0;

            result = if use_multiply {
                match result.checked_mul(next_num) {
                    Some(val) => val,
                    None => break, // Operation would overflow
                }
            } else {
                match result.checked_add(next_num) {
                    Some(val) => val,
                    None => break, // Operation would overflow
                }
            };
        }

        if result == equation.target {
            return true;
        }
    }

    false
}

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_equation() {
        let input = "190: 10 19";
        let eq = parse_equation(input);
        assert_eq!(eq.target, 190);
        assert_eq!(eq.numbers, vec![10, 19]);
    }

    #[test]
    fn test_simple_equation() {
        // 190: 10 19 -> only * works (10 * 19 = 190)
        let eq = Equation {
            target: 190,
            numbers: vec![10, 19],
        };
        assert!(is_solvable(&eq));
    }

    #[test]
    fn test_three_number_equation() {
        // 3267: 81 40 27 -> two solutions
        let eq = Equation {
            target: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(is_solvable(&eq));
    }

    #[test]
    fn test_unsolvable_equation() {
        let eq = Equation {
            target: 100,
            numbers: vec![5, 5],
        };
        assert!(!is_solvable(&eq));
    }

    #[test]
    fn test_multiple_solutions() {
        // 3267: 81 40 27 from example
        let eq = Equation {
            target: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(is_solvable(&eq));
    }
}
