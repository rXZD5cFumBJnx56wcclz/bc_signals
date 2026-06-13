use bc_utils_lg::statics::prices::SRC_NOMAP;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::LazyLock;

use bc_signals::ready::pumpdump::*;
use bc_signals::ready::ready_imports::*;

static SIGNAL: LazyLock<PUMPDUMP> = LazyLock::new(|| PUMPDUMP::new(0.0001, 0.0001, 1.0, 0, 0, 0));
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| SRC_NOMAP.clone());
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    (0..SRC.len())
        .map(|_| vec![Signal::default()])
        .collect::<Vec<Vec<Signal>>>()
});

fn pumpdump_with_bf_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &SRC[SRC.len() - 1];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    let bf = SIGNAL.bf(&*SRC, &*SIGNALS);
    c.bench_function("pumpdump_with_bf", |b| {
        b.iter(|| {
            s.signal_with_bf(
                black_box(src),
                black_box(signals),
                black_box(&bf),
                black_box(0),
            )
        })
    });
}

fn pumpdump_signal_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("pumpdump_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn pumpdump_coll_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("pumpdump_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(
    benches,
    pumpdump_with_bf_1,
    pumpdump_signal_1,
    pumpdump_coll_1
);
criterion_main!(benches);
