
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use bench_align_offset::ALIGN_OFFSET_FNS;

fn bench_align_offset(c: &mut Criterion) {
    let mut group = c.benchmark_group("args");
    for (p, stride) in [(8usize, 24usize)].iter() {
        for align in [16usize, 128, 256, 512, 2048, 4096, 1<<17, 1<<20].iter() {
            let args = (*p, *stride, *align);
            for i in 0..ALIGN_OFFSET_FNS.len() {
                group.bench_with_input(BenchmarkId::new(format!("align_offset_v{}", i), format!("{:?}", args)), &args,
                    |b, args| b.iter(|| unsafe { ALIGN_OFFSET_FNS[i](args.0, args.1, args.2) }));
            }
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
