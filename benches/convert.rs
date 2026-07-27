mod prelude;
use bc_signals::convert::*;
use prelude::*;

static SIGNAL: LazyLock<fn() -> CONVERT> = LazyLock::new(|| || CONVERT);
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 1.0]; 2]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| Default::default());

fn convert_with_bf_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &SRC[SRC.len() - 1];
    let signals = Default::default();
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("convert_with_bf", |b| {
        b.iter(|| s.signal_with_bf(black_box(src), black_box(signals)))
    });
}

fn convert_signal_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("convert_signal_1", |b| {
        b.iter(|| s.signal(black_box(&src), black_box(&signals)))
    });
}

fn convert_coll_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &*SRC;
    let signals = &*SIGNALS;
    c.bench_function("convert_coll_1", |b| {
        b.iter(|| s.signal_coll::<Vec<_>>(black_box(&src), black_box(&signals)))
    });
}

criterion_group!(benches, convert_with_bf_1, convert_signal_1, convert_coll_1);
criterion_main!(benches);
