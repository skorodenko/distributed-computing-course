use nalgebra::{DMatrix};
use lab_4::lineareq::gauss;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};


fn bench_gauss(c: &mut Criterion) {
    let m1 = DMatrix::from_row_slice(2, 3, &[
        1.0, -1.0, -5.0,
        2.0, 1.0, -7.0
    ]);

    let m2 = DMatrix::from_row_slice(3, 4, &[
        2.0, -1.0, 0.0, 0.0,
        -1.0, 1.0, 4.0, 13.0,
        1.0, 2.0, 3.0, 14.0
    ]);

    let m3 = DMatrix::from_row_slice(4, 5, &[
        2.0, 5.0, 4.0, 1.0, 20.0,
        1.0, 3.0, 2.0, 1.0, 11.0,
        2.0, 10.0, 9.0, 7.0, 40.0,
        3.0, 8.0, 9.0, 2.0, 37.0
    ]);
    
    let mut group = c.benchmark_group("Multithread gauss elimination");

    for m in [m1, m2, m3] {
        let signature = format!("input={:?}", m);
        group.bench_with_input(
            BenchmarkId::from_parameter(signature),
            &m,
            |b, inp| {
                b.iter(|| gauss(inp.clone()));
            }
        );
    };
    group.finish();
}

criterion_group!(benches, bench_gauss);
criterion_main!(benches);