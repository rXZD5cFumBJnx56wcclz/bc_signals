use std::sync::LazyLock;

use bc_signals::train::mm::*;
use bc_utils_lg::statics::prices::SRC_NOMAP;

use crate::unit::train::test_funcs::*;

static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| SRC_NOMAP.clone());
const RES: f64 = 0.0;
static SIGNALS: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| Default::default());
static SIGNAL: LazyLock<MM> = LazyLock::new(|| MM::new(0, 0, 2, 3, 0.0001, 0.01, 0.0, 1.0, 2.0));

#[test]
fn mm_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, RES);
}

#[test]
fn mm_signal_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, RES);
}

#[test]
fn mm_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, RES, 30);
}

#[test]
fn mm_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 30);
}

#[test]
fn mm_coll_res_3() {
    test_coll_res_3(
        &*SIGNAL,
        &SRC,
        &SIGNALS,
        vec![
            0.0,
            2.0,
            2.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            1.0,
            1.0,
            1.0,
            0.0,
            2.0,
            0.0,
            0.0,
            2.0,
            0.0,
            0.0,
            2.0,
            2.0,
            2.0,
            0.0,
            1.0,
            0.0,
            2.0,
            2.0,
            0.0,
            1.0,
            0.0,
            0.0,
            1.0,
            1.0,
            1.0,
            0.0,
            2.0,
            0.0,
            1.0,
            1.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            f64::NAN,
            f64::NAN,
            f64::NAN,
        ],
    );
}
