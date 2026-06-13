use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::LazyLock;

use bc_signals::ready::change::*;
use bc_signals::ready::ready_imports::*;

static SIGNAL: LazyLock<CHANGE> = LazyLock::new(|| CHANGE::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| Default::default());
static SIGNALS: LazyLock<Vec<Vec<Signal>>> =
    LazyLock::new(|| vec![vec![Signal::default(), Signal::new(1.0, 1.0)]; 2]);

fn change_with_bf_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = Default::default();
    let signals = &SIGNALS[SIGNALS.len() - 1];
    let bf = SIGNAL.bf(&*SRC, &*SIGNALS);
    c.bench_function("change_with_bf", |b| {
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

fn change_signal_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("change_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn change_coll_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("change_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(benches, change_with_bf_1, change_signal_1, change_coll_1);
criterion_main!(benches);
