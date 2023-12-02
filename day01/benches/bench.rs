use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::{part1, part2, part2a, read_input};

fn criterion_benchmark(c: &mut Criterion) {
    let input = read_input("input.txt");
    c.bench_function("part1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("part2", |b| b.iter(|| part2(black_box(&input))));
    c.bench_function("part2a", |b| b.iter(|| part2a(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
