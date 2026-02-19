use criterion::{Criterion, criterion_group, criterion_main};
use day1::{part_one, part_two};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part_one", |b| {
        b.iter(|| {
            let input = std::fs::read_to_string("input.txt").unwrap();
            part_one(&input)
        })
    });
    c.bench_function("part_two", |b| {
        b.iter(|| {
            let input = std::fs::read_to_string("input.txt").unwrap();
            part_two(&input)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
