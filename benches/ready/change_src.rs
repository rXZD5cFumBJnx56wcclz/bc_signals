use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::LazyLock;

use bc_signals::ready::change_src::*;
use bc_signals::ready::ready_imports::*;

static SIGNAL: LazyLock<CHANGE_SRC> = LazyLock::new(|| CHANGE_SRC::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0], vec![2.0], vec![3.0]]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    let mut a = vec![vec![Signal::new(1.0, 1.0)]; 2];
    a.reserve(1);
    a.push(vec![Signal::new(-1.0, 1.0)]);
    a
});

fn change_src_with_bf_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = SRC[0].as_slice();
    let signals = &SIGNALS[SIGNALS.len() - 1];
    let bf = SIGNAL.bf(&*SRC, &*SIGNALS);
    c.bench_function("change_src_with_bf", |b| {
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

fn change_src_signal_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("change_src_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn change_src_coll_1(c: &mut Criterion) {
    let s = &*SIGNAL;
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("change_src_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(
    benches,
    change_src_with_bf_1,
    change_src_signal_1,
    change_src_coll_1
);
criterion_main!(benches);
