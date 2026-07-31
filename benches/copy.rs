mod prelude;
use bc_signals::copy::*;
use prelude::*;

static SIGNAL: LazyLock<fn() -> COPY> = LazyLock::new(|| || COPY);
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.7333333333333333,]; 2]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| vec![vec![Signal::new(1.0, 1.0)]; 2]);

fn copy_with_bf_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &SRC[SRC.len() - 1];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("copy_with_bf", |b| {
        b.iter(|| s.signal(black_box(src), black_box(signals)))
    });
}

criterion_group!(benches, copy_with_bf_1,);
criterion_main!(benches);
