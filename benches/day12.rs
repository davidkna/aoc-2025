use std::hint::black_box;

use aoc_2025::day12::{INPUT, part_1};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_day_12(c: &mut Criterion) {
    c.bench_function("day-12-part-1", |b| {
        b.iter(|| part_1(black_box(INPUT)));
    });
}

criterion_group!(benches, bench_day_12);
criterion_main!(benches);
