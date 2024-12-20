use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./05-input.txt")?;

    // split input into rules and updates, at the empty line
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // parse rules into a vector of tuples
    let rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once("|").unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();

    // parse updates into a vector of vectors of i32, split by commas
    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    let result = find_valid_updates_middle_sum(&rules, &updates);
    let result2 = find_invalid_updates_middle_sum(&rules, &updates);

    println!("Result: {}", result);
    println!("Result2: {}", result2);
    Ok(())
}

pub fn find_valid_updates_middle_sum(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> i32 {
    updates
        .into_iter()
        .filter(|seq| is_valid_update_sequence(seq, rules))
        .map(|seq| get_middle_number(seq))
        .sum()
}

pub fn find_invalid_updates_middle_sum(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> i32 {
    find_invalid_updates(rules, updates)
        .into_iter()
        .map(|seq| reorder_invalid_update(rules, &seq))
        .map(|seq| get_middle_number(&seq))
        .sum()
}

fn is_valid_update_sequence(seq: &[i32], rules: &[(i32, i32)]) -> bool {
    seq.windows(2).all(|pair| {
        let relevant_rules: Vec<_> = rules
            .iter()
            .filter(|(_, after)| *after == pair[1])
            .map(|(before, _)| before)
            .collect();

        if !relevant_rules.is_empty() {
            return relevant_rules.contains(&&pair[0]);
        }

        false
    })
}

fn get_middle_number(seq: &[i32]) -> i32 {
    seq[seq.len() / 2]
}

fn find_invalid_updates(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> Vec<Vec<i32>> {
    updates
        .into_iter()
        .filter(|seq| !is_valid_update_sequence(seq, rules))
        .cloned()
        .collect()
}

fn reorder_invalid_update(rules: &[(i32, i32)], seq: &[i32]) -> Vec<i32> {
    let mut result = seq.to_vec();
    let mut made_swap = true;

    // Keep swapping until no swaps are made
    while made_swap {
        made_swap = false;

        for i in 0..result.len() - 1 {
            if rules.contains(&(result[i + 1], result[i])) {
                result.swap(i, i + 1);
                made_swap = true;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example data from the puzzle description
    fn example_rules() -> Vec<(i32, i32)> {
        vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]
    }

    fn example_updates() -> Vec<Vec<i32>> {
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
    }

    #[test]
    fn test_find_valid_updates_middle_sum() {
        let rules = example_rules();
        let updates = example_updates();

        assert_eq!(find_valid_updates_middle_sum(&rules, &updates), 143);
    }

    #[test]
    fn test_is_valid_update_sequence() {
        let rules = example_rules();

        // Valid seq #1
        assert!(is_valid_update_sequence(&vec![75, 47, 61, 53, 29], &rules));
        // Valid seq #2
        assert!(is_valid_update_sequence(&vec![97, 61, 53, 29, 13], &rules));
        // Valid seq #3
        assert!(is_valid_update_sequence(&vec![75, 29, 13], &rules));
        // Invalid seq #4
        assert!(!is_valid_update_sequence(&vec![75, 97, 47, 61, 53], &rules));
        // Invalid seq #5
        assert!(!is_valid_update_sequence(&vec![61, 13, 29], &rules));
        // Invalid seq #6
        assert!(!is_valid_update_sequence(&vec![97, 13, 75, 29, 47], &rules));
    }

    #[test]
    fn test_get_middle_number() {
        assert_eq!(get_middle_number(&vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(get_middle_number(&vec![75, 29, 13]), 29);
    }

    #[test]
    fn test_find_invalid_updates_middle_sum() {
        let rules = example_rules();
        let updates = example_updates();

        assert_eq!(find_invalid_updates_middle_sum(&rules, &updates), 123);
    }

    #[test]
    fn test_reorder_invalid_update() {
        let rules = example_rules();

        assert_eq!(
            reorder_invalid_update(&rules, &vec![75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            reorder_invalid_update(&rules, &vec![61, 13, 29]),
            vec![61, 29, 13]
        );
        assert_eq!(
            reorder_invalid_update(&rules, &vec![97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_find_invalid_updates() {
        let rules = example_rules();
        let updates = example_updates();
        let invalid_updates = find_invalid_updates(&rules, &updates);

        assert!(invalid_updates.contains(&vec![75, 97, 47, 61, 53]));
        assert!(invalid_updates.contains(&vec![61, 13, 29]));
        assert!(invalid_updates.contains(&vec![97, 13, 75, 29, 47]));
    }
}
