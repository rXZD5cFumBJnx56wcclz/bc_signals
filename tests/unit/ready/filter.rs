use std::sync::LazyLock;

use bc_signals::ready::filter::*;
use bc_signals::ready::ready_imports::*;

use crate::unit::ready::test_funcs::*;

static SIGNAL: LazyLock<FILTER> = LazyLock::new(|| FILTER::new());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    vec![
        vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
        vec![Signal::new(1.0, 1.0); 2],
    ]
});

#[test]
fn filter_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, RES.clone());
}

#[test]
fn filter_signal_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, RES.clone());
}

#[test]
fn filter_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, RES.clone(), 2);
}

#[test]
fn filter_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
}

#[test]
fn filter_coll_res_3() {
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
                signal: 1.0,
                probability: 1.0,
            },
        ],
    );
}
