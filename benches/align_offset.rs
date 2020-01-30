
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_align_offset::{align_offset_old, align_offset_new};

fn bench_align_offset(c: &mut Criterion) {
    let mut group = c.benchmark_group("args");
    for (p, stride) in [(8usize, 24usize)].iter() {
        for align in [16usize, 32, 64, 128, 256, 512, 1024, 2048, 4096, 1<<17, 1<<20].iter() {
            let tup = (*p, *stride, *align);
            group.bench_with_input(BenchmarkId::new("old", format!("{:?}", tup)), &tup,
                |b, i| b.iter(|| unsafe { align_offset_old(i.0, i.1, i.2) }));
            group.bench_with_input(BenchmarkId::new("new", format!("{:?}", tup)), &tup,
                |b, i| b.iter(|| unsafe { align_offset_new(i.0, i.1, i.2) }));
        }
    }
    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .warm_up_time(core::time::Duration::new(1, 0))
        .sample_size(400);
    targets = bench_align_offset
}
criterion_main!(benches);
