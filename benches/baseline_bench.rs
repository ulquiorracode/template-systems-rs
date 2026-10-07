use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_sample(c: &mut Criterion) {
    c.bench_function("hello_systems_call", |b| {
        b.iter(|| black_box(template_systems_rs::hello_systems()))
    });
}

criterion_group!(benches, benchmark_sample);
criterion_main!(benches);
