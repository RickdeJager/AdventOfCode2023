use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day19::{part1, part2, read_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reading/parsing", |b| {
        b.iter(|| read_input(black_box("input.txt")))
    });

    let input = read_input("input.txt");
    c.bench_function("part1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("part2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
