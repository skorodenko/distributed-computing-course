use std::{sync::Arc, f64::consts::PI};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use lab_3::integration::integral_reduction;


fn bench_integral_reduction(c: &mut Criterion) {
    let f = |x: f64| 1.0 / (f64::sin(2.0*x)).powi(2);
    let a = 0.0;
    let b = PI/2.0;

    let mut group = c.benchmark_group("Multithread integration");
    group.sample_size(10);

    for isteps in [64, 100000, 10000000] {
        let af = Arc::new(f);
        let signature = format!("f=(1/sin(2x)^2), a={}, b={}, steps={}", a, b, isteps);
        let args = (&af, a, b, isteps);
        group.bench_with_input(
            BenchmarkId::from_parameter(signature),
            &args,
            |b, inp| {
                b.iter(|| integral_reduction(inp.0,inp.1,inp.2,inp.3));
            }
        );
    };
    group.finish();
}


criterion_group!(benches, bench_integral_reduction);
criterion_main!(benches);
