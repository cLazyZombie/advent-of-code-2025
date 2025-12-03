//! # Day 3: 배터리 뱅크 최대 전압(Joltage) 계산
//!
//! ## 문제 설명
//! - 각 줄은 하나의 배터리 뱅크를 나타냄 (각 자리는 1-9 사이의 배터리 전압)
//! - Part 1: 각 뱅크에서 정확히 2개의 배터리를 선택하여 2자리 수를 만듦
//! - Part 2: 각 뱅크에서 정확히 12개의 배터리를 선택하여 12자리 수를 만듦
//! - 배터리 순서는 유지해야 함 (재배열 불가)
//! - 각 뱅크에서 만들 수 있는 최대 전압의 총합을 구함

fn main() {
    // input.txt 파일을 컴파일 시점에 문자열로 포함
    let input = include_str!("../input.txt");

    // Part 1: 2개 배터리 선택
    let result1 = solve_part1(input);
    println!("Part 1: {}", result1);

    // Part 2: 12개 배터리 선택
    let result2 = solve_part2(input);
    println!("Part 2: {}", result2);
}

/// Part 1 풀이: 각 뱅크에서 2개 배터리로 만들 수 있는 최대 전압의 합
///
/// # 예시
/// - "987654321111111" → 98 (첫 두 배터리)
/// - "811111111111119" → 89 (8과 9 선택)
fn solve_part1(input: &str) -> u32 {
    input.lines().map(|line| max_joltage(line)).sum()
}

/// Part 2 풀이: 각 뱅크에서 12개 배터리로 만들 수 있는 최대 전압의 합
///
/// # 예시
/// - "987654321111111" → 987654321111
/// - "234234234234278" → 434234234278 (앞의 2,3,2 제외)
fn solve_part2(input: &str) -> u64 {
    input.lines().map(|line| max_joltage_k(line, 12)).sum()
}

/// 문자열에서 k개의 숫자를 순서대로 선택하여 만들 수 있는 최대 수를 반환
///
/// # 알고리즘: 그리디(Greedy) 선택법
///
/// k자리 수를 만들 때, 앞자리가 클수록 전체 수가 커짐.
/// 따라서 각 자리마다 선택 가능한 범위 내에서 가장 큰 숫자를 선택.
///
/// ## 선택 가능한 범위 계산
/// - i번째 자리(0-indexed)를 선택할 때
/// - 시작 위치: 이전에 선택한 위치 + 1
/// - 끝 위치: n - (남은 자리수) = n - (k - i - 1)
///   - 끝 위치 이후에 최소 (k - i - 1)개의 숫자가 남아있어야 함
///
/// ## 예시: "234234234234278" (n=15)에서 k=12개 선택
///
/// ```text
/// 인덱스:  0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
/// 숫자:    2 3 4 2 3 4 2 3 4 2  3  4  2  7  8
///
/// 1번째 자리: 범위 [0, 3], 숫자 2,3,4,2 → 최대 4 (위치 2)
/// 2번째 자리: 범위 [3, 4], 숫자 2,3 → 최대 3 (위치 4)
/// 3번째 자리: 범위 [5, 5], 숫자 4 → 4 (위치 5)
/// ... 이하 생략 ...
/// 결과: 434234234278
/// ```
///
/// # Arguments
/// * `line` - 배터리 뱅크를 나타내는 문자열 (각 문자는 1-9 사이의 숫자)
/// * `k` - 선택할 배터리 개수
///
/// # Returns
/// 선택한 k개의 숫자로 만든 최대 수 (u64)
fn max_joltage_k(line: &str, k: usize) -> u64 {
    // 문자열에서 숫자만 추출하여 벡터로 변환
    // filter_map: 숫자가 아닌 문자는 None으로 필터링됨
    let digits: Vec<u64> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
        .collect();

    // 숫자가 k개 미만이면 선택 불가
    if digits.len() < k {
        return 0;
    }

    let n = digits.len();
    let mut result: u64 = 0; // 최종 결과 (k자리 수)
    let mut start = 0; // 현재 선택 가능한 시작 위치

    // k개의 자리를 순서대로 채움
    for i in 0..k {
        // i번째 자리에서 선택 가능한 끝 위치 계산
        // 남은 자리수: k - i - 1 (현재 자리 제외)
        // 끝 위치 이후에 최소 (k - i - 1)개가 남아야 하므로
        // end = n - (k - i - 1)
        let end = n - (k - i - 1);

        // [start, end) 범위에서 최대 숫자와 그 위치 찾기
        let mut max_digit = 0;
        let mut max_pos = start;
        for pos in start..end {
            if digits[pos] > max_digit {
                max_digit = digits[pos];
                max_pos = pos;
            }
        }

        // 결과에 현재 자리 추가 (10을 곱해서 자리수 이동 후 더함)
        result = result * 10 + max_digit;

        // 다음 선택은 현재 위치 다음부터 시작
        start = max_pos + 1;
    }

    result
}

/// Part 1용: 2개의 숫자를 선택하여 만들 수 있는 최대 2자리 수를 반환
///
/// # 알고리즘: Suffix Maximum 배열 활용
///
/// 2자리 수 XY를 최대화하려면:
/// - X(십의 자리)가 더 중요 → 가능한 큰 X 선택
/// - 각 위치 i에서 X를 선택하면, Y는 i 이후의 최대값이어야 함
///
/// ## Suffix Maximum 배열
/// - suffix_max[i] = max(digits[i], digits[i+1], ..., digits[n-1])
/// - 위치 i 이후(포함)의 최대값을 미리 계산
///
/// ## 최대 전압 계산
/// - 각 위치 i (0 ~ n-2)에서: digits[i] * 10 + suffix_max[i+1]
/// - 이 중 최대값이 정답
///
/// # 예시: "818181911112111"
///
/// ```text
/// 인덱스:     0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
/// 숫자:       8 1 8 1 8 1 9 1 1 1  1  2  1  1  1
/// suffix_max: 9 9 9 9 9 9 9 2 2 2  2  2  1  1  1
///
/// 위치 6에서: 9 * 10 + 2 = 92 (최대!)
/// ```
///
/// # Arguments
/// * `line` - 배터리 뱅크를 나타내는 문자열
///
/// # Returns
/// 선택한 2개의 숫자로 만든 최대 2자리 수 (u32)
fn max_joltage(line: &str) -> u32 {
    // 문자열에서 숫자만 추출
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    // 최소 2개의 숫자가 필요
    if digits.len() < 2 {
        return 0;
    }

    // suffix_max 배열 구축: suffix_max[i] = max(digits[i..])
    // 뒤에서부터 순회하며 계산 (동적 프로그래밍)
    let mut suffix_max = vec![0u32; digits.len()];
    suffix_max[digits.len() - 1] = digits[digits.len() - 1]; // 마지막 원소 초기화

    // 뒤에서 앞으로 순회하며 suffix_max 계산
    // suffix_max[i] = max(digits[i], suffix_max[i+1])
    for i in (0..digits.len() - 1).rev() {
        suffix_max[i] = suffix_max[i + 1].max(digits[i]);
    }

    // 모든 가능한 첫 번째 위치에서 최대 전압 계산
    // 위치 i를 첫 번째 숫자로 선택하면:
    // - 십의 자리: digits[i]
    // - 일의 자리: suffix_max[i+1] (i 이후의 최대값)
    (0..digits.len() - 1)
        .map(|i| digits[i] * 10 + suffix_max[i + 1])
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Part 1 테스트 ====================

    #[test]
    fn test_max_joltage_example1() {
        // "987654321111111"에서 첫 두 숫자 9,8 선택 → 98
        assert_eq!(max_joltage("987654321111111"), 98);
    }

    #[test]
    fn test_max_joltage_example2() {
        // "811111111111119"에서 8과 마지막 9 선택 → 89
        assert_eq!(max_joltage("811111111111119"), 89);
    }

    #[test]
    fn test_max_joltage_example3() {
        // "234234234234278"에서 마지막 7,8 선택 → 78
        assert_eq!(max_joltage("234234234234278"), 78);
    }

    #[test]
    fn test_max_joltage_example4() {
        // "818181911112111"에서 9와 2 선택 → 92
        assert_eq!(max_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_solve_part1_example() {
        // 전체 예시: 98 + 89 + 78 + 92 = 357
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_part1(input), 357);
    }

    // ==================== Part 2 테스트 ====================

    #[test]
    fn test_max_joltage_k_example1() {
        // "987654321111111"에서 12개 선택 → 987654321111
        // 끝의 1 세 개만 제외
        assert_eq!(max_joltage_k("987654321111111", 12), 987654321111);
    }

    #[test]
    fn test_max_joltage_k_example2() {
        // "811111111111119"에서 12개 선택 → 811111111119
        // 중간의 1 세 개 제외, 8과 마지막 9는 반드시 포함
        assert_eq!(max_joltage_k("811111111111119", 12), 811111111119);
    }

    #[test]
    fn test_max_joltage_k_example3() {
        // "234234234234278"에서 12개 선택 → 434234234278
        // 앞의 2,3,2 제외하고 4부터 시작
        assert_eq!(max_joltage_k("234234234234278", 12), 434234234278);
    }

    #[test]
    fn test_max_joltage_k_example4() {
        // "818181911112111"에서 12개 선택 → 888911112111
        // 앞쪽의 1들을 제외하고 8들과 9를 최대한 포함
        assert_eq!(max_joltage_k("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_solve_part2_example() {
        // 전체 예시 합계:
        // 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_part2(input), 3121910778619);
    }
}
