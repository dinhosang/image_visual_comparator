use std::{ffi::OsString, time::Duration};

use clap::Parser;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use image_visual_comparator::{config::AppConfig, run};

fn testing_this_benchmark(c: &mut Criterion) {
    let empty_iter: std::iter::Empty<OsString> = std::iter::empty();

    let mut group = c.benchmark_group("memory");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);

    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = run(AppConfig::parse_from(empty_iter.to_owned()));
            black_box(result);
        })
    });
    group.finish();
}

criterion_group!(benches, testing_this_benchmark);
criterion_main!(benches);
