use bench_align_offset::ALIGN_OFFSET_FNS;
use criterion::{black_box as bb, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_align_offset(c: &mut Criterion) {
    let mut group = c.benchmark_group("args");
    for (p, stride) in [(8usize, 24usize)].iter().copied() {
        for align in [16usize, 128, 256, 512, 2048, 4096, 1 << 17, 1 << 20]
            .iter()
            .copied()
        {
            for i in 0..ALIGN_OFFSET_FNS.len() {
                group.bench_function(
                    BenchmarkId::new(
                        format!("align_offset_v{}", i),
                        format!("({}, {}, {})", p, stride, align),
                    ),
                    |b| b.iter(|| unsafe { ALIGN_OFFSET_FNS[i](bb(p), bb(stride), bb(align)) }),
                );
            }
            for i in 0..ALIGN_OFFSET_FNS.len() {
                group.bench_function(
                    BenchmarkId::new(
                        format!("align_offset_v{}", i),
                        format!("({}, {}, {}*)", p, stride, align),
                    ),
                    |b| b.iter(|| unsafe { ALIGN_OFFSET_FNS[i](bb(p), bb(stride), align) }),
                );
            }
            for i in 0..ALIGN_OFFSET_FNS.len() {
                group.bench_function(
                    BenchmarkId::new(
                        format!("align_offset_v{}", i),
                        format!("({}, {}*, {})", p, stride, align),
                    ),
                    |b| b.iter(|| unsafe { ALIGN_OFFSET_FNS[i](bb(p), stride, bb(align)) }),
                );
            }
            for i in 0..ALIGN_OFFSET_FNS.len() {
                group.bench_function(
                    BenchmarkId::new(
                        format!("align_offset_v{}", i),
                        format!("({}, {}*, {}*)", p, stride, align),
                    ),
                    |b| b.iter(|| unsafe { ALIGN_OFFSET_FNS[i](bb(p), stride, align) }),
                );
            }
        }
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(core::time::Duration::new(1, 0))
        .sample_size(400);
    targets = bench_align_offset
}
criterion_main!(benches);
