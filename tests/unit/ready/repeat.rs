use std::sync::LazyLock;

use bc_signals::ready::ready_imports::*;
use bc_signals::ready::repeat::*;

use crate::unit::ready::test_funcs::*;

static SIGNAL: LazyLock<REPEAT> = LazyLock::new(|| REPEAT::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(0.0, 1.0));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    vec![
        vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
        vec![Signal::new(1.0, 1.0); 2],
    ]
});

#[test]
fn repeat_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn repeat_signal_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn repeat_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
}

#[test]
fn repeat_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
}

#[test]
fn repeat_coll_res_3() {
    test_coll_res_3(
        &*SIGNAL,
        &SRC,
        &SIGNALS,
        vec![
            Signal {
                signal: 0.0,
                probability: 1.0,
            },
            Signal {
                signal: 0.0,
                probability: 1.0,
            },
        ],
    );
}
