use std::fs;

/// Union-Find (Disjoint Set Union) 자료구조
/// 각 노드가 어떤 집합(회로)에 속하는지 추적
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(), // 처음에는 각자가 자신의 부모
            rank: vec![0; n],
        }
    }

    /// x가 속한 집합의 대표(루트) 찾기 (경로 압축 적용)
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 경로 압축
        }
        self.parent[x]
    }

    /// 두 원소를 같은 집합으로 합치기 (rank 기반 union)
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // rank가 낮은 트리를 높은 트리 아래에 붙임
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

/// 입력 문자열을 파싱하여 3D 좌표 리스트 반환
fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

/// 두 점 사이의 유클리드 거리의 제곱 (정렬용으로 제곱 사용 - 루트 불필요)
fn distance_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

/// Part 1 풀이: 1000개의 가장 가까운 쌍을 연결한 후 가장 큰 3개 회로 크기의 곱
fn circuit_part1(input: &str) -> u64 {
    circuit_part1_with_connections(input, 1000)
}

/// Part 2 풀이: 모든 box가 하나의 회로가 될 때까지 연결, 마지막 연결의 X 좌표 곱
fn circuit_part2(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // 모든 쌍의 거리 계산
    let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(&points[i], &points[j]);
            pairs.push((i, j, dist));
        }
    }

    // 거리 기준 오름차순 정렬
    pairs.sort_by_key(|&(_, _, dist)| dist);

    // Union-Find로 회로 추적
    let mut uf = UnionFind::new(n);
    let mut num_circuits = n; // 초기에는 각자 독립된 회로

    // 회로가 1개가 될 때까지 연결
    for &(i, j, _) in &pairs {
        let root_i = uf.find(i);
        let root_j = uf.find(j);

        // 서로 다른 회로인 경우에만 실제 연결 (회로 수 감소)
        if root_i != root_j {
            uf.union(i, j);
            num_circuits -= 1;

            // 모든 box가 하나의 회로가 되면 종료
            if num_circuits == 1 {
                return points[i].0 * points[j].0;
            }
        }
    }

    0 // 도달하지 않음
}

/// 일반화된 풀이: n개의 연결 후 결과 계산
fn circuit_part1_with_connections(input: &str, num_connections: usize) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // 모든 쌍의 거리 계산 (인덱스 i, j와 거리)
    let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(&points[i], &points[j]);
            pairs.push((i, j, dist));
        }
    }

    // 거리 기준 오름차순 정렬
    pairs.sort_by_key(|&(_, _, dist)| dist);

    // Union-Find로 회로 추적
    let mut uf = UnionFind::new(n);

    // 상위 num_connections개 쌍 연결
    for &(i, j, _) in pairs.iter().take(num_connections) {
        uf.union(i, j);
    }

    // 각 회로의 크기 계산
    let mut circuit_sizes: std::collections::HashMap<usize, u64> =
        std::collections::HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    // 크기 내림차순 정렬
    let mut sizes: Vec<u64> = circuit_sizes.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    // 상위 3개 크기의 곱
    sizes.iter().take(3).product()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let result1 = circuit_part1(&input);
    println!("Part 1 결과: {}", result1);

    let result2 = circuit_part2(&input);
    println!("Part 2 결과: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_sample_10_connections() {
        // 10개 연결 후: 5, 4, 2 -> 40
        let result = circuit_part1_with_connections(SAMPLE_INPUT, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_parse_input() {
        let points = parse_input(SAMPLE_INPUT);
        assert_eq!(points.len(), 20);
        assert_eq!(points[0], (162, 817, 812));
        assert_eq!(points[19], (425, 690, 689));
    }

    #[test]
    fn test_sample_part2() {
        // 마지막 연결: 216,146,977 와 117,168,530 -> X좌표 216 * 117 = 25272
        let result = circuit_part2(SAMPLE_INPUT);
        assert_eq!(result, 25272);
    }
}
