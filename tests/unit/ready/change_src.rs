use std::sync::LazyLock;

use bc_signals::ready::change_src::*;
use bc_signals::ready::ready_imports::*;

use crate::unit::ready::test_funcs::*;

static SIGNAL: LazyLock<CHANGE_SRC> = LazyLock::new(|| CHANGE_SRC::default());
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0], vec![2.0], vec![3.0]]);
const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    let mut a = vec![vec![Signal::new(1.0, 1.0)]; 2];
    a.reserve(1);
    a.push(vec![Signal::new(-1.0, 1.0)]);
    a
});

#[test]
fn change_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn change_src_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
}

#[test]
fn change_src_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 3);
}

#[test]
fn change_src_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 3);
}

#[test]
fn change_src_coll_res_3() {
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
