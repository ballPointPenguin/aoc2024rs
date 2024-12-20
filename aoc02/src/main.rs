use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./02-input.txt")?;

    let reports: Vec<Vec<i16>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().expect("Should be a number"))
                .collect()
        })
        .collect();

    let result = count_safe_reports(&reports);
    let result2 = count_safe_with_removal(&reports);

    println!("Result: {}", result);
    println!("Result 2: {}", result2);
}

pub fn count_safe_reports(rows: &[Vec<i16>]) -> i16 {
    rows.iter().filter(|row| is_safe_sequence(row)).count() as i16
}

pub fn count_safe_with_removal(rows: &[Vec<i16>]) -> i16 {
    rows.iter().filter(|row| is_safe_with_removal(row)).count() as i16
}

pub fn is_safe_sequence(report: &[i16]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let first_diff = report[1] - report[0];
    if first_diff.abs() > 3 {
        return false;
    };

    report.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        diff.abs() <= 3 && diff != 0 && diff.signum() == first_diff.signum()
    })
}

pub fn is_safe_with_removal(report: &[i16]) -> bool {
    if is_safe_sequence(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        if is_safe_sequence(&modified) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_reports() -> Vec<Vec<i16>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_single_sequences() {
        // Decreasing by 1-3 (safe)
        assert_eq!(is_safe_sequence(&vec![7, 6, 4, 2, 1]), true);
        // Increasing by 1-3 (safe)
        assert_eq!(is_safe_sequence(&vec![1, 3, 6, 7, 9]), true);
        // Invalid jump (unsafe)
        assert_eq!(is_safe_sequence(&vec![1, 2, 7, 8, 9]), false);
        // Non-monotonic (unsafe)
        assert_eq!(is_safe_sequence(&vec![1, 3, 2, 4, 5]), false);
        // Plateau (unsafe)
        assert_eq!(is_safe_sequence(&vec![8, 6, 4, 4, 1]), false);
    }

    #[test]
    fn test_example_input() {
        let result = count_safe_reports(&example_reports());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_with_removal() {
        let result = count_safe_with_removal(&example_reports());
        assert_eq!(result, 4);
    }
}
