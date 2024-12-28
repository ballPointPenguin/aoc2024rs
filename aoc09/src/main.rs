use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./09-input.txt")?;

    let result = calculate_checksum(&input);
    let result_2 = calculate_checksum_2(&input);

    println!("Result: {}", result);
    println!("Result 2: {}", result_2);

    Ok(())
}

pub fn calculate_checksum(input: &str) -> u64 {
    let disk = compose_file_blocks(input);
    let compacted = compact_disk(&disk);
    calculate_final_checksum(&compacted)
}

pub fn calculate_checksum_2(input: &str) -> u64 {
    let disk = compose_file_blocks(input);
    let compacted = compact_disk_files(&disk);
    calculate_final_checksum(&compacted)
}

fn compose_file_blocks(input: &str) -> Vec<i16> {
    let digits: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let mut output = Vec::new();

    // Enumerate over the digits in chunks of 2
    // For each chunk, the first digit is the file size, the second is the free space
    // make_file with the file_id = chunk_index and file_size = first digit
    // then fill the free space with -1
    for (chunk_index, chunk) in digits.chunks(2).enumerate() {
        let file_size = chunk[0] as usize;
        output.extend(make_file(chunk_index as i16, file_size));

        // Break if there is no chunk[1]
        if chunk.len() < 2 {
            break;
        }

        let free_space = chunk[1] as usize;
        output.extend(make_file(-1, free_space));
    }

    output
}

fn compact_disk(disk: &[i16]) -> Vec<i16> {
    let mut output = Vec::new();
    let mut next_free = 0;
    let mut next_block = disk.len() - 1;

    while next_free <= next_block {
        // Changed condition here
        if disk[next_free] != -1 {
            output.push(disk[next_free]);
            next_free += 1;
        } else {
            while next_block > next_free && disk[next_block] == -1 {
                next_block -= 1;
            }
            if next_block > next_free {
                output.push(disk[next_block]);
                next_block -= 1;
                next_free += 1;
            } else {
                break;
            }
        }
    }

    output
}

fn make_file(file_id: i16, file_size: usize) -> Vec<i16> {
    std::iter::repeat(file_id).take(file_size).collect()
}

fn calculate_final_checksum(compacted: &[i16]) -> u64 {
    compacted
        .iter()
        .enumerate()
        .filter(|(_pos, &digit)| digit >= 0)
        .map(|(pos, &digit)| pos as u64 * digit as u64)
        .sum()
}

fn compact_disk_files(disk: &[i16]) -> Vec<i16> {
    let mut disk = Disk::new(disk);

    for file_idx in 0..disk.files.len() {
        let (_id, start, size) = disk.files[file_idx];
        if let Some(new_pos) = disk.find_free_space(size) {
            // Only move the file if new_pos <= start
            if new_pos <= start {
                disk.move_file(file_idx, new_pos);
            }
        }
    }

    disk.blocks
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<i16>,
    files: Vec<(i16, usize, usize)>,
}

impl Disk {
    fn new(blocks: &[i16]) -> Self {
        let blocks = blocks.to_vec();
        let files = Self::find_files(&blocks);
        Self { blocks, files }
    }

    // Helper function to find contiguous file blocks,
    // returns a list of tuples (file_id, start_index, size)
    fn find_files(blocks: &[i16]) -> Vec<(i16, usize, usize)> {
        let mut files = Vec::new();
        let mut i = 0;

        while i < blocks.len() {
            if blocks[i] != -1 {
                let file_id = blocks[i];
                let start = i;

                while i < blocks.len() && blocks[i] == file_id {
                    i += 1;
                }
                files.push((file_id, start, i - start));
            } else {
                i += 1;
            }
        }

        files.sort_by_key(|&(id, _, _)| -id);
        files
    }

    // Find the leftmost span of free space that can fit size blocks
    fn find_free_space(&self, size: usize) -> Option<usize> {
        let mut count = 0;
        let mut start = None;

        for (i, &block) in self.blocks.iter().enumerate() {
            if block == -1 {
                if start.is_none() {
                    start = Some(i);
                }
                count += 1;
                if count >= size {
                    return start;
                }
            } else {
                count = 0;
                start = None;
            }
        }

        None
    }

    // Move a file to a new position
    fn move_file(&mut self, file_idx: usize, new_pos: usize) {
        let (id, start, size) = self.files[file_idx];

        // Clear the old location
        for i in start..start + size {
            self.blocks[i] = -1;
        }

        // Write to new location
        for i in new_pos..new_pos + size {
            self.blocks[i] = id;
        }

        // Update file record
        self.files[file_idx].1 = new_pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_file_blocks_simple() {
        assert_eq!(compose_file_blocks("1212"), vec![0, -1, -1, 1, -1, -1]);
        assert_eq!(compose_file_blocks("123"), vec![0, -1, -1, 1, 1, 1]);
    }

    #[test]
    fn test_compose_file_blocks_medium() {
        assert_eq!(
            compose_file_blocks("12345"),
            vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2]
        );
    }

    #[test]
    fn test_compose_file_blocks_large() {
        assert_eq!(
            compose_file_blocks("2333133121414131402"),
            vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        );
    }

    #[test]
    fn test_compact_disk_simple() {
        // input: "1212"
        let disk = vec![0, -1, -1, 1, -1, -1];
        assert_eq!(compact_disk(&disk), vec![0, 1]);

        // input: "123"
        let disk = vec![0, -1, -1, 1, 1, 1];
        assert_eq!(compact_disk(&disk), vec![0, 1, 1, 1]);
    }

    #[test]
    fn test_compact_disk_small() {
        // input: "12345"
        let disk = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];
        assert_eq!(compact_disk(&disk), vec![0, 2, 2, 1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn test_compact_disk_medium() {
        // input: "2333133121414131402"
        let disk = vec![
            // 00...111...2...333.44.5555.6666.777.888899
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5,
            -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        assert_eq!(
            compact_disk(&disk),
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
    }

    #[test]
    // Fails assertion
    // left: [2, -1, 0, 0, 3, 3, 3, -1, 1, 1, 1, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]
    // right: [0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5, -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1]
    fn test_compact_disk_files() {
        // input: "2333133121414131402"
        let disk = vec![
            // 00...111...2...333.44.5555.6666.777.888899
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5,
            -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        assert_eq!(
            compact_disk_files(&disk),
            vec![
                0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5,
                -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1
            ]
        );
    }

    #[test]
    fn test_disk_find_files_simple() {
        let blocks = vec![0, -1, -1, 1, 1, 1];
        assert_eq!(Disk::find_files(&blocks), vec![(1, 3, 3), (0, 0, 1)]);
    }

    #[test]
    fn test_disk_find_files_small() {
        // input: "12345"
        let blocks = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];
        assert_eq!(
            Disk::find_files(&blocks),
            vec![(2, 10, 5), (1, 3, 3), (0, 0, 1)]
        );
    }

    #[test]
    fn test_disk_find_files_medium() {
        let blocks = vec![
            // 00...111...2...333.44.5555.6666.777.888899
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5,
            -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        assert_eq!(
            Disk::find_files(&blocks),
            vec![
                (9, 40, 2),
                (8, 36, 4),
                (7, 32, 3),
                (6, 27, 4),
                (5, 22, 4),
                (4, 19, 2),
                (3, 15, 3),
                (2, 11, 1),
                (1, 5, 3),
                (0, 0, 2)
            ]
        );
    }

    #[test]
    fn test_disk_find_free_space_small() {
        // input: "12345"
        let disk = Disk::new(&vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2]);

        assert_eq!(disk.find_free_space(1), Some(1));
        assert_eq!(disk.find_free_space(2), Some(1));
        assert_eq!(disk.find_free_space(3), Some(6));
        assert_eq!(disk.find_free_space(4), Some(6));
        assert_eq!(disk.find_free_space(5), None);
    }

    #[test]
    fn test_disk_find_free_space_medium() {
        // input: "2333133121414131402"
        let disk = Disk::new(&vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ]);

        assert_eq!(disk.find_free_space(1), Some(2));
        assert_eq!(disk.find_free_space(2), Some(2));
        assert_eq!(disk.find_free_space(3), Some(2));
        assert_eq!(disk.find_free_space(4), None);
        assert_eq!(disk.find_free_space(5), None);
    }

    #[test]
    fn test_calculate_final_checksum() {
        // 0099811188827773336446555566..............
        let compacted = vec![
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
        ];
        assert_eq!(calculate_final_checksum(&compacted), 1928);
    }

    #[test]
    fn test_checksum_single_file() {
        assert_eq!(calculate_checksum("5"), 0);
    }

    #[test]
    fn test_checksum_alternating_files() {
        assert_eq!(calculate_checksum("11111"), 4);
    }

    #[test]
    fn test_checksum_simple_disk_map() {
        assert_eq!(calculate_checksum("12345"), 60);
    }

    #[test]
    fn test_checksum_no_free_space() {
        assert_eq!(calculate_checksum("90909"), 513);
    }

    #[test]
    fn test_checksum_puzzle_example() {
        let input = "2333133121414131402";
        assert_eq!(calculate_checksum(input), 1928);
    }

    #[test]
    // Fails: attempt to add with overflow (main.rs:86:5, calculate_final_checksum)
    fn test_checksum_puzzle_example_2() {
        let input = "2333133121414131402";
        assert_eq!(calculate_checksum_2(input), 2858);
    }
}
