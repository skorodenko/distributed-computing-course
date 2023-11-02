use ndarray::prelude::*;
use lab_4::lineareq::gauss;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};


fn bench_gauss(c: &mut Criterion) {
    let mut group = c.benchmark_group("Multithread gauss elimination");

    group.sample_size(10);

    for mm in [10, 100, 1000] {
        for nworkers in [1,2,4,8] {
            let m = Array::<f64, _>::random((mm,mm+1), Uniform::new(0.,10.));
            let pool = rayon::ThreadPoolBuilder::new().num_threads(nworkers).build().unwrap();
            let signature = format!("mm={}, nworkers={}", mm, nworkers);
            group.bench_with_input(
                BenchmarkId::from_parameter(signature),
                &m,
                |b, m| {
                    b.iter(|| pool.install(|| gauss(m.clone())));
                }
            );
        };
    };
    group.finish();
}

criterion_group!(benches, bench_gauss);
criterion_main!(benches);