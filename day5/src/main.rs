use std::ops::RangeInclusive;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    let ranges_section = input.split_once("\n\n").map(|(r, _)| r).unwrap_or(input);
    ranges_section
        .lines()
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            s.parse().unwrap()..=e.parse().unwrap()
        })
        .collect()
}

fn fresh_part1(input: &str) -> usize {
    let (ranges_section, ids_section) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<u64>> = ranges_section
        .lines()
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            s.parse().unwrap()..=e.parse().unwrap()
        })
        .collect();

    ids_section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

fn fresh_part2(input: &str) -> u64 {
    let ranges = parse_ranges(input);

    // 시작점 기준 정렬을 위해 (start, end) 튜플로 변환
    let mut intervals: Vec<(u64, u64)> = ranges
        .iter()
        .map(|r| (*r.start(), *r.end()))
        .collect();
    intervals.sort_by_key(|&(s, _)| s);

    // 겹치거나 인접한 범위 병합
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in intervals {
        if let Some(last) = merged.last_mut() {
            // 현재 범위가 이전 범위와 겹치거나 인접하면 병합
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // 병합된 범위들의 크기 합산
    merged.iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_part1() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!(fresh_part1(input), 3);
    }

    #[test]
    fn test_fresh_part2() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!(fresh_part2(input), 14);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", fresh_part1(input));
    println!("Part 2: {}", fresh_part2(input));
}
