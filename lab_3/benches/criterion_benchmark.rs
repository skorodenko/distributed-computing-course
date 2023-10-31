use std::{sync::Arc, f64::consts::PI};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use lab_3::integration::integral_reduction;


fn bench_integral_reduction(c: &mut Criterion) {
    let f = |x: f64| 1.0 / (f64::sin(2.0*x)).powi(2);
    let a = 0.0;
    let b = PI/2.0;

    let mut group = c.benchmark_group("Multithread integration");

    for isteps in [64, 1e5 as i32, 1e7 as i32] {
        for nworkers in [1,2,4,8] {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(nworkers).build().unwrap();
            let af = Arc::new(f);
            let signature = format!("f=(1/sin(2x)^2), a={}, b={}, nsteps={}, nworkers={}", a, b, isteps, nworkers);
            let args = (&af, a, b, isteps);
            group.bench_with_input(
                BenchmarkId::from_parameter(signature),
                &args,
                |b, inp| {
                    b.iter(|| pool.install(|| integral_reduction(inp.0,inp.1,inp.2,inp.3)));
                }
            );
        };
    };
    group.finish();
}


criterion_group!(benches, bench_integral_reduction);
criterion_main!(benches);
