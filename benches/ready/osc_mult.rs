use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::LazyLock;

use bc_signals::ready::osc_mult::*;
use bc_signals::ready::ready_imports::*;

static SIGNAL: LazyLock<OSC_MULT> = LazyLock::new(|| OSC_MULT::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.04,]; 2]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| vec![vec![Signal::new(1.0, 1.0)]; 2]);

fn osc_mult_with_bf_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &SRC[SRC.len() - 1];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    let bf = SIGNAL.bf(&*SRC, &*SIGNALS);
    c.bench_function("osc_mult_with_bf", |b| {
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

fn osc_mult_signal_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("osc_mult_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn osc_mult_coll_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("osc_mult_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(
    benches,
    osc_mult_with_bf_1,
    osc_mult_signal_1,
    osc_mult_coll_1
);
criterion_main!(benches);
