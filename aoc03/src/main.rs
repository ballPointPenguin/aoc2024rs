use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

lazy_static! {
    static ref INSTRUCTION_RE: Regex =
        Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("./03-input.txt")?;

    let result = sum_multiplications(&input);
    let result2 = sum_multiplications_v2(&input);

    println!("Result: {}", result);
    println!("Result 2: {}", result2);

    Ok(())
}

pub fn collect_numbers(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .filter_map(|cap| {
            let x = cap[1].parse::<u32>().ok()?;
            let y = cap[2].parse::<u32>().ok()?;
            Some((x, y))
        })
        .collect()
}

pub fn sum_multiplications(input: &str) -> u32 {
    let num_pairs: Vec<(u32, u32)> = collect_numbers(input);

    num_pairs.iter().map(|(x, y)| x * y).sum()
}

pub fn sum_multiplications_v2(input: &str) -> u32 {
    let mut enabled = true;
    let mut sum = 0;

    for cap in INSTRUCTION_RE.captures_iter(input) {
        let instruction = cap.get(0).unwrap().as_str();

        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if instruction.starts_with("mul") => {
                if enabled {
                    if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                        if let (Ok(x), Ok(y)) =
                            (x.as_str().parse::<u32>(), y.as_str().parse::<u32>())
                        {
                            sum += x * y
                        }
                    }
                }
            }
            _ => {}
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_collect_numbers() {
        let expected: Vec<(u32, u32)> = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(collect_numbers(INPUT), expected);
    }

    #[test]
    fn test_sum_multiplications() {
        assert_eq!(sum_multiplications(INPUT), 161);
    }

    #[test]
    fn test_digit_length_constraints() {
        let input = "mul(1234,5)mul(1,5678)mul(999,999)mul(0,42)";
        let expected: Vec<(u32, u32)> = vec![(999, 999), (0, 42)];
        assert_eq!(collect_numbers(input), expected);
    }

    #[test]
    fn test_edge_cases() {
        let input = "mul(,4)mul(3,)mul()mul(1!)mul(123,456!)";
        let expected: Vec<(u32, u32)> = vec![]; // None of these should match
        assert_eq!(collect_numbers(input), expected);
    }

    #[test]
    fn test_sum_multiplications_with_state() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(sum_multiplications_v2(input), 48);
    }

    #[test]
    fn test_state_changes() {
        // Start enabled (default state)
        let input1 = "mul(2,3)don't()mul(4,5)";
        assert_eq!(sum_multiplications_v2(input1), 6); // only 2*3 counts

        // Test re-enabling
        let input2 = "mul(2,3)don't()mul(4,5)do()mul(6,7)";
        assert_eq!(sum_multiplications_v2(input2), 48); // 2*3 + 6*7
    }
}
