use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::LazyLock;

use bc_signals::ready::ready_imports::*;
use bc_signals::ready::repeat::*;

static SIGNAL: LazyLock<REPEAT> = LazyLock::new(|| REPEAT::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    vec![
        vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
        vec![Signal::new(1.0, 1.0); 2],
    ]
});

fn repeat_with_bf_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &vec![];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    let bf = SIGNAL.bf(&*SRC, &*SIGNALS);
    c.bench_function("repeat_with_bf", |b| {
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

fn repeat_signal_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("repeat_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn repeat_coll_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("repeat_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(benches, repeat_with_bf_1, repeat_signal_1, repeat_coll_1);
criterion_main!(benches);
