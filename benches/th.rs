use bc_utils_lg::statics::prices::SRC;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::LazyLock;

use bc_signals::prelude::*;
use bc_signals::th::*;

static SIGNAL: LazyLock<TH> = LazyLock::new(|| TH::new(0.0001, 0.0001, 1.0, 0, 0, 0, 0., -1., 1.));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    (0..SRC.len())
        .map(|_| vec![Signal::default()])
        .collect::<Vec<Vec<Signal>>>()
});

fn th_with_bf_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &SRC[SRC.len() - 1];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    let bf = SIGNAL.bf(&*SRC, &*SIGNALS);
    c.bench_function("th_with_bf", |b| {
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

fn th_signal_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("th_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn th_coll_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("th_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(benches, th_with_bf_1, th_signal_1, th_coll_1);
criterion_main!(benches);
