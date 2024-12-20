use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./04-input.txt")?;

    let result = count_xmas(&input);
    let result2 = count_masx(&input);

    println!("Result: {}", result);
    println!("Result 2: {}", result2);

    Ok(())
}

pub fn count_xmas(input: &str) -> usize {
    let rows = get_rows(input);
    let cols = get_cols(&rows);
    let diags = get_diagonals(&rows);

    let rows_owned: Vec<String> = rows.into_iter().map(String::from).collect();

    [rows_owned, cols, diags]
        .concat()
        .into_iter()
        .map(|slice| slice.matches("XMAS").count() + slice.matches("SAMX").count())
        .sum()
}

fn get_rows(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_cols(rows: &[&str]) -> Vec<String> {
    let width = rows[0].len();
    (0..width)
        .map(|i| {
            rows.iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect()
}

fn get_diagonals(rows: &[&str]) -> Vec<String> {
    let height = rows.len();
    let width = rows[0].len();
    let mut diagonals = Vec::new();

    let get_char = |row: usize, col: usize| rows[row].chars().nth(col).unwrap();

    // Get diagonals starting from top row (both directions)
    for start_col in 0..width {
        // Left to right
        let lr_diag: String = (0..height.min(width - start_col))
            .map(|offset| get_char(offset, start_col + offset))
            .collect();
        diagonals.push(lr_diag);

        // Right to left
        let rl_diag: String = (0..height.min(start_col + 1))
            .map(|offset| get_char(offset, start_col - offset))
            .collect();
        diagonals.push(rl_diag);
    }

    // Get diagonals starting from first column (excluding top row, both directions)
    for start_row in 1..height {
        let max_diagonal_len = (height - start_row).min(width);

        // Left to right
        let lr_diag: String = (0..max_diagonal_len)
            .map(|offset| get_char(start_row + offset, offset))
            .collect();
        diagonals.push(lr_diag);

        // Right to left
        let rl_diag: String = (0..max_diagonal_len)
            .map(|offset| get_char(start_row + offset, width - 1 - offset))
            .collect();
        diagonals.push(rl_diag);
    }

    diagonals
}

pub fn count_masx(input: &str) -> usize {
    let rows = get_rows(input);

    center_positions(&rows)
        .into_iter()
        .filter(|center| is_valid_pattern(&rows, center))
        .count()
}

fn center_positions(rows: &[&str]) -> Vec<(usize, usize)> {
    rows.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'A')
                .map(move |(col, _)| (row, col))
        })
        .collect()
}

fn is_valid_pattern(rows: &[&str], center: &(usize, usize)) -> bool {
    let (row, col) = *center;

    if row == 0 || row >= rows.len() - 1 || col == 0 || col >= rows[0].len() - 1 {
        return false;
    }

    let [prev, next] = [rows[row - 1].as_bytes(), rows[row + 1].as_bytes()];

    let neighbors = [
        prev[col - 1] as char,
        prev[col + 1] as char,
        next[col - 1] as char,
        next[col + 1] as char,
    ];

    is_mas(&neighbors)
}

fn is_mas(group: &[char]) -> bool {
    const PATTERNS: [[char; 4]; 4] = [
        ['M', 'M', 'S', 'S'],
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
        ['S', 'S', 'M', 'M'],
    ];

    PATTERNS.iter().any(|p| p == group)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const SMALL_INPUT: &str = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    #[test]
    fn test_get_rows() {
        let rows = get_rows(SMALL_INPUT);

        assert_eq!(rows[0], "..X...");
    }

    #[test]
    fn test_get_cols() {
        let rows = get_rows(SMALL_INPUT);
        let cols = get_cols(&rows);

        assert_eq!(cols[0], "...X.");
    }

    #[test]
    fn get_diagonals_test() {
        let rows = get_rows(SMALL_INPUT);
        let diags = get_diagonals(&rows);

        assert_eq!(diags.len(), 20);
        assert_eq!(diags[0], ".S.S.");
        assert_eq!(diags[11], ".X.AX");
        assert_eq!(diags[12], ".AA.");
        assert_eq!(diags[13], ".AS.");
    }

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas(SAMPLE_INPUT), 18);
    }

    #[test]
    fn test_center_positions() {
        use std::collections::HashSet;

        let rows = get_rows(SAMPLE_INPUT);
        let expected: HashSet<(usize, usize)> = vec![
            (1, 2),
            (2, 6),
            (2, 7),
            (3, 2),
            (3, 4),
            (7, 1),
            (7, 3),
            (7, 5),
            (7, 7),
        ]
        .into_iter()
        .collect();

        let actual: HashSet<_> = center_positions(&rows).into_iter().collect();

        assert!(expected.is_subset(&actual));
    }

    #[test]
    fn test_is_valid_pattern() {
        let rows = get_rows(SAMPLE_INPUT);

        assert!(is_valid_pattern(&rows, &(1, 2)));
        assert!(!is_valid_pattern(&rows, &(2, 1)));
    }

    #[test]
    fn test_is_mas() {
        assert!(is_mas(&['M', 'M', 'S', 'S']));
        assert!(is_mas(&['M', 'S', 'M', 'S']));
        assert!(!is_mas(&['M', 'S', 'S', 'M']));
        assert!(!is_mas(&['.', '.', '.', '.']));
    }

    #[test]
    fn test_count_masx() {
        assert_eq!(count_masx(SAMPLE_INPUT), 9);
    }
}
