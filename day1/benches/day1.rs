use criterion::{criterion_group, criterion_main, Criterion};
use day1::{eleven_method};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("eleven_method", |b| b.iter(|| {
        let input = std::fs::read_to_string("input.txt").unwrap();
        eleven_method(&input)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
