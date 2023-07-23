use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use image_visual_comparator::run;

fn testing_this_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);

    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = run();
            black_box(result);
        })
    });
    group.finish();
}

criterion_group!(benches, testing_this_benchmark);
criterion_main!(benches);
