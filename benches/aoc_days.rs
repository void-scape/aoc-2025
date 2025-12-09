use aoc_2025::days;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    bench(
        c,
        include_str!("../inputs/1.txt"),
        1,
        days::one::part_one,
        days::one::part_two,
    );
    bench(
        c,
        include_str!("../inputs/2.txt"),
        2,
        days::two::part_one,
        days::two::part_two,
    );
    bench(
        c,
        include_str!("../inputs/3.txt"),
        3,
        days::three::part_one_bench,
        days::three::part_two_bench,
    );
    bench(
        c,
        include_str!("../inputs/4.txt"),
        4,
        days::four::part_one_bench,
        days::four::part_two_bench,
    );
    bench(
        c,
        include_str!("../inputs/5.txt"),
        5,
        days::five::part_one_bench,
        days::five::part_two_bench,
    );
    bench(
        c,
        include_str!("../inputs/6.txt"),
        6,
        days::six::part_one_bench,
        days::six::part_two_bench,
    );
    bench(
        c,
        include_str!("../inputs/7.txt"),
        7,
        days::seven::part_one_bench,
        days::seven::part_two_bench,
    );
    bench(
        c,
        include_str!("../inputs/8.txt"),
        8,
        days::eight::part_one_bench,
        days::eight::part_two_bench,
    );
}

fn bench<O1, O2>(
    c: &mut Criterion,
    input: &str,
    day: usize,
    part_one: impl Fn(&str) -> O1,
    part_two: impl Fn(&str) -> O2,
) {
    c.bench_function(&format!("{} p1", day), |b| {
        b.iter(|| {
            let result = part_one(input);
            black_box(result);
        })
    });
    c.bench_function(&format!("{} p2", day), |b| {
        b.iter(|| {
            let result = part_two(input);
            black_box(result);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
