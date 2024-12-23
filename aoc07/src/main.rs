use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./07-input.txt")?;

    let result = total_calibration_result(&input);
    let result2 = total_calibration_result_concat(&input);

    println!("Result: {}", result);
    println!("Result2: {}", result2);
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

pub fn total_calibration_result_concat(input: &str) -> i64 {
    input
        .lines()
        .map(|line| parse_equation(line))
        .filter(|eq| is_solvable_concat(eq))
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

fn is_solvable_concat(equation: &Equation) -> bool {
    let num_operators = equation.numbers.len() - 1;
    let combinations = 3_i64.pow(num_operators as u32);

    for i in 0..combinations {
        let mut result = equation.numbers[0];
        let mut combo = i;
        let mut valid_sequence = true;

        for pos in 0..num_operators {
            let next_num = equation.numbers[pos + 1];
            // Get rightmost trit (0 = add, 1 = multiply, 2 = concat)
            let operator = combo % 3;
            combo /= 3;

            let next_result = match operator {
                0 => match result.checked_add(next_num) {
                    Some(sum) if sum > equation.target => {
                        valid_sequence = false;
                        break;
                    }
                    Some(sum) => sum,
                    None => {
                        valid_sequence = false;
                        break;
                    }
                },
                1 => match result.checked_mul(next_num) {
                    Some(product) if product > equation.target => {
                        valid_sequence = false;
                        break;
                    }
                    Some(product) => product,
                    None => {
                        valid_sequence = false;
                        break;
                    }
                },
                2 => match concatenate(result, next_num) {
                    Some(concatenated) if concatenated > equation.target => {
                        valid_sequence = false;
                        break;
                    }
                    Some(concatenated) => concatenated,
                    None => {
                        valid_sequence = false;
                        break;
                    }
                },
                _ => unreachable!(),
            };

            result = next_result;
        }

        if valid_sequence && result == equation.target {
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

fn concatenate(a: i64, b: i64) -> Option<i64> {
    let b_digits = if b == 0 {
        1
    } else {
        (b as f64).log10().floor() as u32 + 1
    };

    // a * 10^(digits in b) + b
    let multiplier = 10_i64.checked_pow(b_digits)?;
    a.checked_mul(multiplier)?.checked_add(b)
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

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(12, 345), Some(12345));
        assert_eq!(concatenate(6, 8), Some(68));
        assert_eq!(concatenate(15, 6), Some(156));
    }

    #[test]
    fn test_concat_equations() {
        // Test cases from part 2
        assert!(is_solvable_concat(&Equation {
            target: 156,
            numbers: vec![15, 6]
        }));

        assert!(is_solvable_concat(&Equation {
            target: 7290,
            numbers: vec![6, 8, 6, 15]
        }));

        assert!(is_solvable_concat(&Equation {
            target: 192,
            numbers: vec![17, 8, 14]
        }));
    }

    #[test]
    fn test_calibration_sums() {
        let input = "\
190: 10 19
3267: 81 40 27
292: 11 6 16 20
156: 15 6
7290: 6 8 6 15
192: 17 8 14";

        assert_eq!(total_calibration_result(input), 3749); // Part 1
        assert_eq!(total_calibration_result_concat(input), 11387); // Part 2
    }

    #[test]
    fn test_edge_case_equations() {
        // Test large numbers that might cause overflow issues
        let large = Equation {
            target: 999999999,
            numbers: vec![999, 999, 999],
        };
        assert!(is_solvable_concat(&large));

        // Test where concatenation could produce a larger intermediate
        // result than the target but subsequent operations might fix it
        let intermediate = Equation {
            target: 100,
            numbers: vec![99, 99, 1],
        };
        assert!(!is_solvable_concat(&intermediate));

        // Test where only very specific operator combinations work
        let specific = Equation {
            target: 1234,
            numbers: vec![12, 34, 56],
        };
        // We should verify this expected result carefully
        assert!(!is_solvable_concat(&specific));
    }
}
