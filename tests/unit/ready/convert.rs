use std::sync::LazyLock;

use bc_signals::ready::convert::*;
use bc_signals::ready::ready_imports::*;

use crate::unit::ready::test_funcs::*;

static SIGNAL: LazyLock<CONVERT> = LazyLock::new(|| CONVERT::new());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 1.0]; 2]);
const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| Default::default());

#[test]
fn convert_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn convert_signal_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn convert_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
}

#[test]
fn convert_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
}

#[test]
fn convert_coll_res_3() {
    test_coll_res_3(
        &*SIGNAL,
        &SRC,
        &SIGNALS,
        vec![
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
        ],
    );
}
