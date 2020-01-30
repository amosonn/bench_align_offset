
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_align_offset::{align_offset_old, align_offset_new};

fn bench_align_offset(c: &mut Criterion) {
    let mut group = c.benchmark_group("args");
    for p in [1usize, 3, 37].iter() {
        for stride in [3usize, 8, 10].iter() {
            for align in [1usize, 2, 4, 8, 16, 32, 64, 128, 256, 512].iter() {
                let tup = (*p, *stride, *align);
                group.bench_with_input(BenchmarkId::new("old", format!("{:?}", tup)), &tup,
                    |b, i| b.iter(|| unsafe { align_offset_old(i.0, i.1, i.2) }));
                group.bench_with_input(BenchmarkId::new("new", format!("{:?}", tup)), &tup,
                    |b, i| b.iter(|| unsafe { align_offset_new(i.0, i.1, i.2) }));
            }
        }
    }
    group.finish();
}

criterion_group!(benches, bench_align_offset);
criterion_main!(benches);
