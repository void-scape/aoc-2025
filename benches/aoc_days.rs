use aoc_2025::days;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    two(c);
}

#[allow(unused)]
fn two(c: &mut Criterion) {
    c.bench_function("2 p1", |b| {
        b.iter(|| {
            let input = include_str!("../inputs/2.txt");
            let result = days::two::part_one(input);
            black_box(result);
        })
    });
    c.bench_function("2 p2", |b| {
        b.iter(|| {
            let input = include_str!("../inputs/2.txt");
            let result = days::two::part_two(input);
            black_box(result);
        })
    });
}

#[allow(unused)]
fn one(c: &mut Criterion) {
    c.bench_function("1 p1", |b| {
        b.iter(|| {
            let input = include_str!("../inputs/1.txt");
            let result = days::one::part_one(input);
            black_box(result);
        })
    });
    c.bench_function("1 p2", |b| {
        b.iter(|| {
            let input = include_str!("../inputs/1.txt");
            let result = days::one::part_two(input);
            black_box(result);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
