
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_align_offset::{ver1, ver2};

fn bench_align_offset(c: &mut Criterion) {
    let mut group = c.benchmark_group("args");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("old", i), i, 
            |b, i| b.iter(|| ver1(*i)));
        group.bench_with_input(BenchmarkId::new("new", i), i, 
            |b, i| b.iter(|| ver2(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench_align_offset);
criterion_main!(benches);
