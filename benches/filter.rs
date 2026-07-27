mod prelude;
use bc_signals::filter::*;
use prelude::*;

static SIGNAL: LazyLock<fn() -> FILTER> = LazyLock::new(|| || FILTER);
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    vec![
        vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
        vec![Signal::new(1.0, 1.0); 2],
    ]
});

fn filter_with_bf_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = Default::default();
    let signals = &SIGNALS[SIGNALS.len() - 1];
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("filter_with_bf", |b| {
        b.iter(|| s.signal_with_bf(black_box(src), black_box(signals)))
    });
}

fn filter_signal_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("filter_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn filter_coll_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("filter_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(benches, filter_with_bf_1, filter_signal_1, filter_coll_1);
criterion_main!(benches);
