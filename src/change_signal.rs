#![allow(non_camel_case_types)]

use crate::prelude::*;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct CHANGE_SIGNAL {
    signal_l: RefCell<f64>,
    signal_l_state: RefCell<f64>,
}

impl W for CHANGE_SIGNAL {
    fn w(&self) -> usize {
        2
    }
}

impl SignalReady for CHANGE_SIGNAL {
    fn init_bf(&self, _src: &[Vec<f64>], signals: &[Vec<Signal>]) {
        *self.signal_l.borrow_mut() = signals[signals.len() - 1][0].signal;
        *self.signal_l_state.borrow_mut() = *self.signal_l.borrow();
    }
    fn execute_bf(&self) {
        *self.signal_l.borrow_mut() = *self.signal_l_state.borrow();
    }
    fn signal(&self, _src: &[f64], signals: &[Signal]) -> Signal {
        let signal = *signals.get(0).expect("signal not found");
        *self.signal_l_state.borrow_mut() = signal.signal;
        if signal.signal != *self.signal_l.borrow() {
            signal
        } else {
            Default::default()
        }
    }
}

impl SignalReadyExt for CHANGE_SIGNAL {}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::prelude_tests::prelude::*;

    static SIGNAL: LazyLock<fn() -> CHANGE_SIGNAL> = LazyLock::new(|| || CHANGE_SIGNAL::default());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(-1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        let mut a = vec![vec![Signal::new(1.0, 1.0)]; 2];
        a.reserve(1);
        a.push(vec![Signal::new(-1.0, 1.0)]);
        a
    });

    #[test]
    fn change_signal_res_1() {
        test_bf_res_1(&SIGNAL(), &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn change_signal_coll_res_1() {
        test_coll_res_1(&SIGNAL(), &SRC, &SIGNALS, 1);
    }
}
