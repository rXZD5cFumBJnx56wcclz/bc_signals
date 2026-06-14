use std::sync::LazyLock;

use bc_signals::ready::osc_mult::*;
use bc_signals::ready::ready_imports::*;

use crate::unit::ready::test_funcs::*;

static SIGNAL: LazyLock<OSC_MULT> = LazyLock::new(|| OSC_MULT::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.04,]; 2]);
const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 0.7333333333333333));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| vec![vec![Signal::new(1.0, 1.0)]; 2]);

#[test]
fn osc_mult_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn osc_mult_signal_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn osc_mult_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
}

#[test]
fn osc_mult_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
}

#[test]
fn osc_mult_coll_res_3() {
    test_coll_res_3(
        &*SIGNAL,
        &SRC,
        &SIGNALS,
        vec![
            Signal {
                signal: 1.0,
                probability: 0.7333333333333333,
            },
            Signal {
                signal: 1.0,
                probability: 0.7333333333333333,
            },
        ],
    );
}
