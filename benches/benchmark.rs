use criterion::{criterion_group, criterion_main, Criterion};
use regex::is_match;
use std::time::Duration;

const INPUTS: &[(&str, &str, &str)] = &[
    ("n = 2", "a?a?aa", "aa"),
    ("n = 4", "a?a?a?a?aaaa", "aaaa"),
    ("n = 8", "a?a?a?a?a?a?a?a?aaaaaaaa", "aaaaaaaa"),
    (
        "n = 16",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
    ),
];

fn depth_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("Depth First");
    g.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| is_match(args.0, args.1, true))
        });
    }
}

fn width_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("Width First");
    g.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| is_match(args.0, args.1, false))
        });
    }
}

criterion_group!(benches, depth_first, width_first);
criterion_main!(benches);
