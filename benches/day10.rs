use std::hint::black_box;

use aoc_2025::day10::{INPUT, part_1, part_2};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_day_10(c: &mut Criterion) {
    c.bench_function("day-10-part-1", |b| {
        b.iter(|| part_1(black_box(INPUT)));
    });

    c.bench_function("day-10-part-2", |b| {
        b.iter(|| part_2(black_box(INPUT)));
    });
}

criterion_group!(benches, bench_day_10);
criterion_main!(benches);
