use std::sync::LazyLock;

use bc_signals::ready::peak::*;
use bc_signals::ready::ready_imports::*;
use bc_utils_lg::statics::prices::SRC_NOMAP;

use crate::unit::ready::test_funcs::*;

static SIGNAL: LazyLock<PEAK> = LazyLock::new(|| PEAK::new(0.0001, 0.0001, 1.0, 0, 0, 0));
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| SRC_NOMAP.clone());
const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
    (0..SRC.len())
        .map(|_| vec![Signal::default()])
        .collect::<Vec<Vec<Signal>>>()
});

#[test]
fn peak_with_bf_res_1() {
    test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, RES.clone());
}

#[test]
fn peak_signal_res_1() {
    test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, RES.clone());
}

#[test]
fn peak_coll_res_1() {
    test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, RES.clone(), 30);
}

#[test]
fn peak_coll_res_2() {
    test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 30);
}

#[test]
fn peak_coll_res_3() {
    test_coll_res_3(
        &*SIGNAL,
        &SRC,
        &SIGNALS,
        vec![
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 0.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 0.0,
                probability: 1.0,
            },
            Signal {
                signal: 0.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 0.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
            Signal {
                signal: -1.0,
                probability: 1.0,
            },
            Signal {
                signal: 1.0,
                probability: 1.0,
            },
        ],
    );
}
