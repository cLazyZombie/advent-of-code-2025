use day2::{sum_invalid_part1_optimized, sum_invalid_part2_optimized};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt 읽기 실패");
    let input = input.trim();

    // 범위 파싱
    let ranges: Vec<(u64, u64)> = input
        .split(',')
        .map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            let start: u64 = parts[0].parse().expect("시작 숫자 파싱 실패");
            let end: u64 = parts[1].parse().expect("끝 숫자 파싱 실패");
            (start, end)
        })
        .collect();

    // 최적화 알고리즘 사용
    let sum_part1: u64 = ranges
        .iter()
        .map(|&(start, end)| sum_invalid_part1_optimized(start, end))
        .sum();
    let sum_part2: u64 = ranges
        .iter()
        .map(|&(start, end)| sum_invalid_part2_optimized(start, end))
        .sum();

    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
}
