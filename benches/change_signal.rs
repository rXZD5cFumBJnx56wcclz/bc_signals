mod prelude;
use bc_signals::change_signal::*;
use prelude::*;

static SIGNAL: LazyLock<fn() -> CHANGE_SIGNAL> = LazyLock::new(|| || CHANGE_SIGNAL::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| Default::default());
static SIGNALS: LazyLock<Vec<Vec<Signal>>> =
    LazyLock::new(|| vec![vec![Signal::default(), Signal::new(1.0, 1.0)]; 2]);

fn change_signal_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = Default::default();
    let signals = &SIGNALS[SIGNALS.len() - 1];
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("change_signal", |b| {
        b.iter(|| s.signal(black_box(src), black_box(signals)))
    });
}

criterion_group!(benches, change_signal_1,);
criterion_main!(benches);
