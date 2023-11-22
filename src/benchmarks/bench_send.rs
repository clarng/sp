use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod transport;

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("your_function", |b| b.iter(|| your_function(black_box(42))));
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);