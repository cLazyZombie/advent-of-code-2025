use std::collections::HashSet;
use std::fs;

/// Part 1: 숫자가 "무효 ID"인지 확인 (패턴이 정확히 2번 반복)
/// 예: 11, 6464, 123123
#[cfg(test)]
fn is_invalid_part1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // 짝수 길이여야 2번 반복 패턴 가능
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

/// Part 2: 숫자가 "무효 ID"인지 확인 (패턴이 2번 이상 반복)
/// 예: 12341234 (2회), 123123123 (3회), 1212121212 (5회), 1111111 (7회)
#[cfg(test)]
fn is_invalid_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // 가능한 모든 패턴 길이 시도 (1부터 len/2까지)
    for pattern_len in 1..=len / 2 {
        if len.is_multiple_of(pattern_len) {
            let pattern = &s[..pattern_len];
            let repeats = len / pattern_len;
            if repeats >= 2 && pattern.repeat(repeats) == s {
                return true;
            }
        }
    }
    false
}

// ========== 최적화 버전 ==========

/// Part 1 최적화: 등차수열 공식으로 범위 내 무효 ID 합계 계산
/// O(range_size) 대신 O(1)
fn sum_invalid_part1_optimized(start: u64, end: u64) -> u64 {
    let mut sum = 0u64;

    // 가능한 모든 패턴 길이 k에 대해
    for k in 1..=10u32 {
        let multiplier = 10u64.pow(k) + 1; // 예: k=2 -> 101, k=3 -> 1001

        // 패턴은 정확히 k자리여야 함 (앞에 0 불가)
        let pattern_min_natural = if k == 1 { 1 } else { 10u64.pow(k - 1) };
        let pattern_max_natural = 10u64.pow(k) - 1;

        // 범위 제약에서 패턴 경계 계산
        let pattern_min_from_range = start.div_ceil(multiplier);
        let pattern_max_from_range = end / multiplier;

        let p_min = pattern_min_natural.max(pattern_min_from_range);
        let p_max = pattern_max_natural.min(pattern_max_from_range);

        if p_min <= p_max {
            let count = p_max - p_min + 1;
            let pattern_sum = (p_min + p_max) * count / 2; // 등차수열 합
            sum += pattern_sum * multiplier;
        }
    }

    sum
}

/// Part 2 최적화: 범위 내 모든 무효 ID 찾기 (패턴 2회 이상 반복)
/// 범위 내 모든 숫자 대신 패턴만 열거
fn sum_invalid_part2_optimized(start: u64, end: u64) -> u64 {
    let mut invalid_ids = HashSet::new();

    // 각 패턴 길이 p에 대해
    for p in 1..=10u32 {
        // 패턴은 정확히 p자리여야 함 (앞에 0 불가)
        let pattern_min = if p == 1 { 1 } else { 10u64.pow(p - 1) };
        let pattern_max = 10u64.pow(p) - 1;

        // 각 반복 횟수 r에 대해 (최소 2회)
        for r in 2..=20u32 {
            if p * r > 20 {
                break; // u64 최대 약 20자리
            }

            // 승수 계산: 10^(p*(r-1)) + 10^(p*(r-2)) + ... + 1
            let multiplier: u64 = (0..r).map(|i| 10u64.pow(p * i)).sum();

            // [start, end] 범위에서 유효한 패턴 범위 찾기
            let p_min_from_range = start.div_ceil(multiplier);
            let p_max_from_range = end / multiplier;

            let p_min = pattern_min.max(p_min_from_range);
            let p_max = pattern_max.min(p_max_from_range);

            if p_min <= p_max {
                for pattern in p_min..=p_max {
                    let id = pattern * multiplier;
                    invalid_ids.insert(id);
                }
            }
        }
    }

    invalid_ids.iter().sum()
}

/// Part 1 브루트포스 합계 (비교용)
#[cfg(test)]
fn sum_invalid_part1_bruteforce(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_invalid_part1(n)).sum()
}

/// Part 2 브루트포스 합계 (비교용)
#[cfg(test)]
fn sum_invalid_part2_bruteforce(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_invalid_part2(n)).sum()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Part 1 테스트 ==========

    #[test]
    fn test_is_invalid_part1() {
        // 무효 ID (2번 반복 패턴)
        assert!(is_invalid_part1(11));
        assert!(is_invalid_part1(22));
        assert!(is_invalid_part1(99));
        assert!(is_invalid_part1(6464));
        assert!(is_invalid_part1(1010));
        assert!(is_invalid_part1(123123));
        assert!(is_invalid_part1(222222));
        assert!(is_invalid_part1(446446));
        assert!(is_invalid_part1(1188511885));

        // 유효 ID (2번 반복 아님)
        assert!(!is_invalid_part1(12));
        assert!(!is_invalid_part1(101)); // 홀수 길이
        assert!(!is_invalid_part1(111)); // 3회 반복, 2회 아님
        assert!(!is_invalid_part1(1234));
        assert!(!is_invalid_part1(1698522));
    }

    #[test]
    fn test_part1_range_11_22() {
        let invalid: Vec<u64> = (11..=22).filter(|&n| is_invalid_part1(n)).collect();
        assert_eq!(invalid, vec![11, 22]);
    }

    #[test]
    fn test_part1_range_95_115() {
        let invalid: Vec<u64> = (95..=115).filter(|&n| is_invalid_part1(n)).collect();
        assert_eq!(invalid, vec![99]);
    }

    #[test]
    fn test_part1_range_998_1012() {
        let invalid: Vec<u64> = (998..=1012).filter(|&n| is_invalid_part1(n)).collect();
        assert_eq!(invalid, vec![1010]);
    }

    // ========== Part 2 테스트 ==========

    #[test]
    fn test_is_invalid_part2() {
        // 패턴 2회 이상 반복
        assert!(is_invalid_part2(11)); // 1 × 2
        assert!(is_invalid_part2(111)); // 1 × 3
        assert!(is_invalid_part2(1111111)); // 1 × 7
        assert!(is_invalid_part2(6464)); // 64 × 2
        assert!(is_invalid_part2(123123)); // 123 × 2
        assert!(is_invalid_part2(123123123)); // 123 × 3
        assert!(is_invalid_part2(1212121212)); // 12 × 5
        assert!(is_invalid_part2(12341234)); // 1234 × 2
        assert!(is_invalid_part2(565656)); // 56 × 3
        assert!(is_invalid_part2(824824824)); // 824 × 3
        assert!(is_invalid_part2(2121212121)); // 21 × 5

        // 반복 패턴 아님
        assert!(!is_invalid_part2(12));
        assert!(!is_invalid_part2(123));
        assert!(!is_invalid_part2(1234));
        assert!(!is_invalid_part2(1698522));
    }

    #[test]
    fn test_part2_range_11_22() {
        // 무효 ID 2개: 11, 22
        let invalid: Vec<u64> = (11..=22).filter(|&n| is_invalid_part2(n)).collect();
        assert_eq!(invalid, vec![11, 22]);
    }

    #[test]
    fn test_part2_range_95_115() {
        // 무효 ID 2개: 99, 111
        let invalid: Vec<u64> = (95..=115).filter(|&n| is_invalid_part2(n)).collect();
        assert_eq!(invalid, vec![99, 111]);
    }

    #[test]
    fn test_part2_range_998_1012() {
        // 무효 ID 2개: 999, 1010
        let invalid: Vec<u64> = (998..=1012).filter(|&n| is_invalid_part2(n)).collect();
        assert_eq!(invalid, vec![999, 1010]);
    }

    #[test]
    fn test_part2_range_1188511880_1188511890() {
        // 무효 ID 1개: 1188511885
        let invalid: Vec<u64> = (1188511880..=1188511890)
            .filter(|&n| is_invalid_part2(n))
            .collect();
        assert_eq!(invalid, vec![1188511885]);
    }

    #[test]
    fn test_part2_range_222220_222224() {
        // 무효 ID 1개: 222222
        let invalid: Vec<u64> = (222220..=222224).filter(|&n| is_invalid_part2(n)).collect();
        assert_eq!(invalid, vec![222222]);
    }

    #[test]
    fn test_part2_range_1698522_1698528() {
        // 무효 ID 없음
        let invalid: Vec<u64> = (1698522..=1698528)
            .filter(|&n| is_invalid_part2(n))
            .collect();
        assert!(invalid.is_empty());
    }

    #[test]
    fn test_part2_range_446443_446449() {
        // 무효 ID 1개: 446446
        let invalid: Vec<u64> = (446443..=446449).filter(|&n| is_invalid_part2(n)).collect();
        assert_eq!(invalid, vec![446446]);
    }

    #[test]
    fn test_part2_range_38593856_38593862() {
        // 무효 ID 1개: 38593859
        let invalid: Vec<u64> = (38593856..=38593862)
            .filter(|&n| is_invalid_part2(n))
            .collect();
        assert_eq!(invalid, vec![38593859]);
    }

    #[test]
    fn test_part2_range_565653_565659() {
        // 무효 ID 1개: 565656
        let invalid: Vec<u64> = (565653..=565659).filter(|&n| is_invalid_part2(n)).collect();
        assert_eq!(invalid, vec![565656]);
    }

    #[test]
    fn test_part2_range_824824821_824824827() {
        // 무효 ID 1개: 824824824
        let invalid: Vec<u64> = (824824821..=824824827)
            .filter(|&n| is_invalid_part2(n))
            .collect();
        assert_eq!(invalid, vec![824824824]);
    }

    #[test]
    fn test_part2_range_2121212118_2121212124() {
        // 무효 ID 1개: 2121212121
        let invalid: Vec<u64> = (2121212118..=2121212124)
            .filter(|&n| is_invalid_part2(n))
            .collect();
        assert_eq!(invalid, vec![2121212121]);
    }

    // ========== 최적화 vs 브루트포스 비교 테스트 ==========

    #[test]
    fn test_optimized_part1_small_ranges() {
        let ranges = [
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
        ];
        for (start, end) in ranges {
            let brute = sum_invalid_part1_bruteforce(start, end);
            let opt = sum_invalid_part1_optimized(start, end);
            assert_eq!(brute, opt, "Part1 불일치: 범위 {}-{}", start, end);
        }
    }

    #[test]
    fn test_optimized_part2_small_ranges() {
        let ranges = [
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];
        for (start, end) in ranges {
            let brute = sum_invalid_part2_bruteforce(start, end);
            let opt = sum_invalid_part2_optimized(start, end);
            assert_eq!(brute, opt, "Part2 불일치: 범위 {}-{}", start, end);
        }
    }

    #[test]
    fn test_optimized_part1_medium_ranges() {
        let ranges = [
            (1, 1000),
            (10000, 20000),
            (100000, 110000),
            (999000, 1001000),
        ];
        for (start, end) in ranges {
            let brute = sum_invalid_part1_bruteforce(start, end);
            let opt = sum_invalid_part1_optimized(start, end);
            assert_eq!(brute, opt, "Part1 불일치: 범위 {}-{}", start, end);
        }
    }

    #[test]
    fn test_optimized_part2_medium_ranges() {
        let ranges = [
            (1, 1000),
            (10000, 20000),
            (100000, 110000),
            (999000, 1001000),
        ];
        for (start, end) in ranges {
            let brute = sum_invalid_part2_bruteforce(start, end);
            let opt = sum_invalid_part2_optimized(start, end);
            assert_eq!(brute, opt, "Part2 불일치: 범위 {}-{}", start, end);
        }
    }
}
