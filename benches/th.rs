mod prelude;
use bc_signals::th::*;
use prelude::*;

static SIGNAL: LazyLock<fn() -> TH> =
    LazyLock::new(|| || TH::new(0.0001, 0.0001, 1.0, 1, 1, 0, 0., -1., 1.));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    (0..SRC.len())
        .map(|_| vec![Signal::default()])
        .collect::<Vec<Vec<Signal>>>()
});

fn th_with_bf_1(c: &mut Criterion) {
    let s = SIGNAL();
    let src = &SRC[SRC.len() - 1];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("th_with_bf", |b| {
        b.iter(|| s.signal(black_box(src), black_box(signals)))
    });
}

criterion_group!(benches, th_with_bf_1,);
criterion_main!(benches);
