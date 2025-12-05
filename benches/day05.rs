use std::hint::black_box;

use aoc_2025::day05::{INPUT, parse_input, part_1, part_2};
use bstr::ByteSlice;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_day_05(c: &mut Criterion) {
    c.bench_function("day-05-part-1", |b| {
        b.iter(|| part_1(black_box(INPUT)));
    });

    c.bench_function("day-05-part-2", |b| {
        b.iter(|| part_2(black_box(INPUT)));
    });

    c.bench_function("day-05-parse-input", |b| {
        let relevant_input = INPUT.split_once_str("\n\n").unwrap().0;
        b.iter(|| parse_input(black_box(relevant_input)));
    });
}

criterion_group!(benches, bench_day_05);
criterion_main!(benches);
