use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("01-input.txt")?;

    let (list1, list2): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .filter_map(|line| {
            let nums: Result<Vec<i32>, _> =
                line.split_whitespace().take(2).map(|s| s.parse()).collect();

            nums.ok().filter(|v| v.len() == 2)
        })
        .fold((Vec::new(), Vec::new()), |(mut v1, mut v2), nums| {
            v1.push(nums[0]);
            v2.push(nums[1]);
            (v1, v2)
        });

    let answer = process_lists(&list1, &list2);
    println!("Answer: {}", answer);

    let answer2 = similarity_score(&list1, &list2);
    println!("Answer 2: {}", answer2);

    Ok(())
}

fn process_lists(list1: &[i32], list2: &[i32]) -> i32 {
    let mut vec1 = list1.to_vec();
    let mut vec2 = list2.to_vec();
    vec1.sort();
    vec2.sort();

    vec1.into_iter().zip(vec2).map(|(a, b)| (a - b).abs()).sum()
}

fn similarity_score(list1: &[i32], list2: &[i32]) -> i32 {
    let mut frequency = HashMap::new();
    for &n in list2 {
        *frequency.entry(n).or_insert(0) += 1;
    }

    list1
        .iter()
        .map(|&n| n * frequency.get(&n).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(process_lists(&list1, &list2), 11);
    }

    #[test]
    fn test_similarity_score() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(&list1, &list2), 31);
    }
}
