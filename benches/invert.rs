mod prelude;
use bc_signals::invert::*;
use prelude::*;

static SIGNAL: LazyLock<INVERT> = LazyLock::new(|| INVERT::default());
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| vec![vec![Signal::new(1., 1.),]; 3]);

fn invert_with_bf_1(c: &mut Criterion) {
    let s = SIGNAL.clone();
    let src = &SRC[SRC.len() - 1];
    let signals = &SIGNALS[SIGNALS.len() - 1];
    s.init_bf(&*SRC, &*SIGNALS);
    c.bench_function("invert_with_bf", |b| {
        b.iter(|| s.signal(black_box(src), black_box(signals)))
    });
}

criterion_group!(benches, invert_with_bf_1,);
criterion_main!(benches);
