//! # Day 4: Printing Department
//!
//! ## 문제 설명
//! - 종이 롤(@)이 격자에 배치되어 있음
//! - 지게차가 종이 롤에 접근 가능한 조건: 8방향 이웃 중 종이 롤이 4개 미만일 때
//! - Part 1: 접근 가능한 종이 롤의 개수를 구함
//! - Part 2: 반복적으로 제거하여 총 제거 가능한 롤 수를 구함

/// 8방향 이웃 오프셋 (상하좌우 + 대각선)
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let input = include_str!("../input.txt");

    let result1 = paper_part1(input);
    println!("Part 1: {}", result1);

    let result2 = paper_part2(input);
    println!("Part 2: {}", result2);
}

/// Part 1: 지게차가 접근 가능한 종이 롤 개수
///
/// # 알고리즘
/// 1. 입력을 2D 격자로 파싱
/// 2. 각 '@' 위치에서 8방향 이웃 확인
/// 3. 이웃 중 '@' 개수가 4 미만이면 접근 가능
///
/// # Arguments
/// * `input` - 격자 형태의 입력 문자열 (`.`은 빈 공간, `@`는 종이 롤)
///
/// # Returns
/// 접근 가능한 종이 롤의 개수
fn paper_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    find_accessible(&grid).len()
}

/// Part 2: 반복적으로 제거하여 총 제거 가능한 롤 수
///
/// # 알고리즘
/// 1. 접근 가능한 롤 찾기
/// 2. 모두 제거
/// 3. 더 이상 제거할 수 없을 때까지 반복
/// 4. 총 제거 수 반환
fn paper_part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total_removed = 0;

    loop {
        let accessible = find_accessible(&grid);

        if accessible.is_empty() {
            break;
        }

        // 접근 가능한 롤 제거
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }

        total_removed += accessible.len();
    }

    total_removed
}

/// 격자에서 접근 가능한 롤의 위치를 반환
///
/// 접근 가능 조건: 8방향 이웃 중 '@'가 4개 미만
fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    if grid.is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '@' {
                continue;
            }

            let neighbor_count = count_neighbors(grid, row, col);

            if neighbor_count < 4 {
                accessible.push((row, col));
            }
        }
    }

    accessible
}

/// 특정 위치의 8방향 이웃 중 '@' 개수를 카운트
fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for (dr, dc) in &DIRECTIONS {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_part1_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(paper_part1(input), 13);
    }

    #[test]
    fn test_paper_part1_single_roll() {
        // 단일 종이 롤은 이웃이 0개이므로 접근 가능
        let input = "...
.@.
...";
        assert_eq!(paper_part1(input), 1);
    }

    #[test]
    fn test_paper_part1_surrounded() {
        // 중앙 롤은 이웃 8개, 코너 4개는 이웃 3개씩
        // 가운데 행 양쪽은 이웃 5개씩
        let input = "@@@
@@@
@@@";
        // 코너 4개: 이웃 3개 < 4 → 접근 가능
        // 가운데 4개(가장자리): 이웃 5개 >= 4 → 접근 불가
        // 중앙 1개: 이웃 8개 >= 4 → 접근 불가
        assert_eq!(paper_part1(input), 4);
    }

    #[test]
    fn test_paper_part1_empty() {
        let input = "...
...
...";
        assert_eq!(paper_part1(input), 0);
    }

    // ==================== Part 2 테스트 ====================

    #[test]
    fn test_paper_part2_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(paper_part2(input), 43);
    }

    #[test]
    fn test_paper_part2_all_removable() {
        // 모든 롤이 제거 가능한 경우
        let input = "@.@
...
@.@";
        // 각 롤은 이웃 0개 → 4개 모두 제거 가능
        assert_eq!(paper_part2(input), 4);
    }

    #[test]
    fn test_paper_part2_chain_removal() {
        // 연쇄 제거 테스트
        let input = "@@@@@";
        // 첫 라운드: 양 끝 2개 제거 (이웃 1개씩)
        // 두 번째: 새로운 양 끝 2개 제거
        // 세 번째: 마지막 1개 제거
        // 총 5개
        assert_eq!(paper_part2(input), 5);
    }
}
