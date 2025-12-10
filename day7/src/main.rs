use std::collections::{HashMap, HashSet};
use std::fs;

fn tachyon_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find starting position S
    let mut start_col = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = col_idx;
                break;
            }
        }
        if grid[row_idx].contains(&'S') {
            break;
        }
    }

    // Track active beam positions (columns) for current row
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut split_count = 0;

    // Process each row starting from row after S
    for row in 1..rows {
        // New beam positions after processing this row
        let mut new_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            let ch = grid[row][col];
            if ch == '^' {
                // Splitter: beam splits left and right
                split_count += 1;
                if col > 0 {
                    new_beams.insert(col - 1);
                }
                if col + 1 < cols {
                    new_beams.insert(col + 1);
                }
            } else {
                // Empty space or other: beam continues downward
                new_beams.insert(col);
            }
        }

        beams = new_beams;
        if beams.is_empty() {
            break;
        }
    }

    split_count
}

fn tachyon_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find starting position S
    let mut start_col = 0;
    for row in &grid {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = col_idx;
                break;
            }
        }
    }

    // Track timeline counts at each column position
    let mut timelines: HashMap<usize, usize> = HashMap::new();
    timelines.insert(start_col, 1);

    // Process each row starting from row after S
    for row in 1..rows {
        let mut new_timelines: HashMap<usize, usize> = HashMap::new();

        for (&col, &count) in &timelines {
            let ch = grid[row][col];
            if ch == '^' {
                // Splitter: timeline splits left and right
                if col > 0 {
                    *new_timelines.entry(col - 1).or_insert(0) += count;
                }
                if col + 1 < cols {
                    *new_timelines.entry(col + 1).or_insert(0) += count;
                }
            } else {
                // Empty space: timeline continues downward
                *new_timelines.entry(col).or_insert(0) += count;
            }
        }

        timelines = new_timelines;
        if timelines.is_empty() {
            break;
        }
    }

    // Sum all timeline counts
    timelines.values().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part 1: {}", tachyon_part1(&input));
    println!("Part 2: {}", tachyon_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(tachyon_part1(input), 21);
    }

    #[test]
    fn test_part2_sample() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(tachyon_part2(input), 40);
    }
}
