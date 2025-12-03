use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use day2::{
    sum_invalid_part1_bruteforce, sum_invalid_part1_optimized, sum_invalid_part2_bruteforce,
    sum_invalid_part2_optimized,
};

/// 다양한 범위 크기로 Part 1 벤치마크
fn bench_part1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Part1");

    // 작은 범위 (12개)
    let small_range = (11u64, 22u64);
    group.bench_with_input(
        BenchmarkId::new("bruteforce", "small(12)"),
        &small_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part1_bruteforce(black_box(start), black_box(end))),
    );
    group.bench_with_input(
        BenchmarkId::new("optimized", "small(12)"),
        &small_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part1_optimized(black_box(start), black_box(end))),
    );

    // 중간 범위 (1,000개)
    let medium_range = (1u64, 1000u64);
    group.bench_with_input(
        BenchmarkId::new("bruteforce", "medium(1K)"),
        &medium_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part1_bruteforce(black_box(start), black_box(end))),
    );
    group.bench_with_input(
        BenchmarkId::new("optimized", "medium(1K)"),
        &medium_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part1_optimized(black_box(start), black_box(end))),
    );

    // 큰 범위 (100,000개)
    let large_range = (1u64, 100_000u64);
    group.bench_with_input(
        BenchmarkId::new("bruteforce", "large(100K)"),
        &large_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part1_bruteforce(black_box(start), black_box(end))),
    );
    group.bench_with_input(
        BenchmarkId::new("optimized", "large(100K)"),
        &large_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part1_optimized(black_box(start), black_box(end))),
    );

    group.finish();
}

/// 다양한 범위 크기로 Part 2 벤치마크
fn bench_part2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Part2");

    // 작은 범위 (12개)
    let small_range = (11u64, 22u64);
    group.bench_with_input(
        BenchmarkId::new("bruteforce", "small(12)"),
        &small_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part2_bruteforce(black_box(start), black_box(end))),
    );
    group.bench_with_input(
        BenchmarkId::new("optimized", "small(12)"),
        &small_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part2_optimized(black_box(start), black_box(end))),
    );

    // 중간 범위 (1,000개)
    let medium_range = (1u64, 1000u64);
    group.bench_with_input(
        BenchmarkId::new("bruteforce", "medium(1K)"),
        &medium_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part2_bruteforce(black_box(start), black_box(end))),
    );
    group.bench_with_input(
        BenchmarkId::new("optimized", "medium(1K)"),
        &medium_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part2_optimized(black_box(start), black_box(end))),
    );

    // 큰 범위 (100,000개)
    let large_range = (1u64, 100_000u64);
    group.bench_with_input(
        BenchmarkId::new("bruteforce", "large(100K)"),
        &large_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part2_bruteforce(black_box(start), black_box(end))),
    );
    group.bench_with_input(
        BenchmarkId::new("optimized", "large(100K)"),
        &large_range,
        |b, &(start, end)| b.iter(|| sum_invalid_part2_optimized(black_box(start), black_box(end))),
    );

    group.finish();
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
