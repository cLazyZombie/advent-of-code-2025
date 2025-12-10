fn tiles_part1(input: &str) -> i64 {
    let points: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn tiles_part2(input: &str) -> i64 {
    let red_tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    // 수직/수평 세그먼트 수집
    let mut vertical_segments: Vec<(i64, i64, i64)> = vec![]; // (x, y_min, y_max)
    let mut horizontal_segments: Vec<(i64, i64, i64)> = vec![]; // (y, x_min, x_max)

    for i in 0..red_tiles.len() {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % red_tiles.len()];

        if x1 == x2 {
            let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            vertical_segments.push((x1, y_min, y_max));
        } else {
            let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            horizontal_segments.push((y1, x_min, x_max));
        }
    }

    // 경계 위인지 확인
    let on_boundary = |x: i64, y: i64| -> bool {
        for &(vx, vy_min, vy_max) in &vertical_segments {
            if x == vx && y >= vy_min && y <= vy_max {
                return true;
            }
        }
        for &(hy, hx_min, hx_max) in &horizontal_segments {
            if y == hy && x >= hx_min && x <= hx_max {
                return true;
            }
        }
        false
    };

    // 점이 폴리곤 내부(경계 포함)인지 확인 (ray casting)
    let is_inside_or_boundary = |x: i64, y: i64| -> bool {
        if on_boundary(x, y) {
            return true;
        }
        // ray casting: 오른쪽으로 반직선
        let mut count = 0;
        for &(vx, vy_min, vy_max) in &vertical_segments {
            // 오른쪽에 있고, y가 세그먼트 범위 내 (위 끝점 포함, 아래 끝점 제외)
            if vx > x && y >= vy_min && y < vy_max {
                count += 1;
            }
        }
        count % 2 == 1
    };

    // 세그먼트가 직사각형 내부(경계 제외)를 지나는지 확인
    let segment_crosses_interior = |x1: i64, y1: i64, x2: i64, y2: i64| -> bool {
        let (rx_min, rx_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (ry_min, ry_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        // 수평 세그먼트가 직사각형 내부를 지나는지
        for &(y, x_min, x_max) in &horizontal_segments {
            if y > ry_min && y < ry_max {
                // x 범위가 직사각형 내부와 겹치는지
                if x_min.max(rx_min) < x_max.min(rx_max) {
                    return true;
                }
            }
        }

        // 수직 세그먼트가 직사각형 내부를 지나는지
        for &(x, y_min, y_max) in &vertical_segments {
            if x > rx_min && x < rx_max {
                // y 범위가 직사각형 내부와 겹치는지
                if y_min.max(ry_min) < y_max.min(ry_max) {
                    return true;
                }
            }
        }

        false
    };

    // 모든 빨간 타일 쌍에 대해
    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            // 나머지 두 꼭지점이 내부(경계 포함)인지 확인
            if !is_inside_or_boundary(x1, y2) || !is_inside_or_boundary(x2, y1) {
                continue;
            }

            // 세그먼트가 직사각형 내부를 지나는지 확인
            if segment_crosses_interior(x1, y1, x2, y2) {
                continue;
            }

            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn main() {
    // Sample test
    let sample = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let sample_result1 = tiles_part1(sample);
    println!("Sample Part 1: {} (expected: 50)", sample_result1);
    assert_eq!(sample_result1, 50, "Sample Part 1 failed!");

    let sample_result2 = tiles_part2(sample);
    println!("Sample Part 2: {} (expected: 24)", sample_result2);
    assert_eq!(sample_result2, 24, "Sample Part 2 failed!");

    // Real input
    let input = include_str!("../input.txt");
    let result1 = tiles_part1(input);
    println!("Part 1: {}", result1);

    let result2 = tiles_part2(input);
    println!("Part 2: {}", result2);
}
