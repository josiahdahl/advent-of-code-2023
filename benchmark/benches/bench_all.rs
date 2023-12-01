use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01a", |b| b.iter(|| day01a::main()));
    c.bench_function("day01b", |b| b.iter(|| day01b::main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
