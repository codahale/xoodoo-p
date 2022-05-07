use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use xoodoo_p::xoodoo;

fn permutation_benchmarks(c: &mut Criterion) {
    let mut g = c.benchmark_group("permutation");
    g.sample_size(1_000);
    g.throughput(Throughput::Bytes(48));
    g.bench_function("Xoodoo", |b| {
        let mut lanes = [0u32; 12];
        b.iter(|| xoodoo::<12>(&mut lanes))
    });
    g.finish();
}

criterion_group!(benches, permutation_benchmarks);
criterion_main!(benches);
