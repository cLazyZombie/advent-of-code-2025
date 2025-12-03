use std::collections::HashSet;

/// Part 1: 숫자가 "무효 ID"인지 확인 (패턴이 정확히 2번 반복)
/// 예: 11, 6464, 123123
pub fn is_invalid_part1(n: u64) -> bool {
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
pub fn is_invalid_part2(n: u64) -> bool {
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

// ========== 브루트포스 버전 ==========

/// Part 1 브루트포스: 범위 내 모든 숫자 순회
pub fn sum_invalid_part1_bruteforce(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_invalid_part1(n)).sum()
}

/// Part 2 브루트포스: 범위 내 모든 숫자 순회
pub fn sum_invalid_part2_bruteforce(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_invalid_part2(n)).sum()
}

// ========== 최적화 버전 ==========

/// Part 1 최적화: 등차수열 공식으로 범위 내 무효 ID 합계 계산
/// O(range_size) 대신 O(1)
pub fn sum_invalid_part1_optimized(start: u64, end: u64) -> u64 {
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
pub fn sum_invalid_part2_optimized(start: u64, end: u64) -> u64 {
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

    // ========== 최적화 vs 브루트포스 비교 테스트 ==========

    #[test]
    fn test_optimized_vs_bruteforce() {
        let ranges = [
            (11, 22),
            (95, 115),
            (998, 1012),
            (1, 1000),
            (10000, 20000),
        ];
        for (start, end) in ranges {
            assert_eq!(
                sum_invalid_part1_bruteforce(start, end),
                sum_invalid_part1_optimized(start, end),
                "Part1 불일치: 범위 {}-{}",
                start,
                end
            );
            assert_eq!(
                sum_invalid_part2_bruteforce(start, end),
                sum_invalid_part2_optimized(start, end),
                "Part2 불일치: 범위 {}-{}",
                start,
                end
            );
        }
    }
}
