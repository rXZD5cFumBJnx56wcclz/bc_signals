mod prelude;
use bc_signals::repeat::*;
use prelude::*;

static SIGNAL: LazyLock<fn() -> REPEAT> = LazyLock::new(|| || REPEAT::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    vec![
        vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
        vec![Signal::new(1.0, 1.0); 2],
    ]
});

fn repeat_with_bf_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &vec![];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("repeat_with_bf", |b| {
        b.iter(|| s.signal(black_box(src), black_box(signals)))
    });
}

criterion_group!(benches, repeat_with_bf_1,);
criterion_main!(benches);
