#![allow(non_camel_case_types)]

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CHANGE_SIGNAL {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl CHANGE_SIGNAL {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 1,
            add_window_accuracy: 2,
        }
    }
    pub fn set_window(
        &mut self,
        window: usize,
    ) {
        self.window = window;
    }
    pub fn set_mult_window_accuracy(
        &mut self,
        mult_window_accuracy: usize,
    ) {
        self.mult_window_accuracy = mult_window_accuracy;
    }
    pub fn set_add_window_accuracy(
        &mut self,
        add_window_accuracy: usize,
    ) {
        self.add_window_accuracy = add_window_accuracy;
    }
}

impl Default for CHANGE_SIGNAL {
    fn default() -> Self {
        CHANGE_SIGNAL::new()
    }
}

impl SignalReady for CHANGE_SIGNAL {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(
        &self,
        _: &[Vec<f64>],
        signals: &[Vec<Signal>],
    ) -> BF_SIGNALS<'a> {
        <BF_SIGNALS as BfSignalsExt>::new([(
            "signal_l",
            vec![vec![signals[signals.len() - 1][0].signal]],
        )])
    }
    fn signal_with_bf<'a>(
        &self,
        _: &[f64],
        signals: &[Signal],
        bf: &BF_SIGNALS<'a>,
        index_: usize,
    ) -> Signal {
        let signal = *signals.get(0).expect("signal not found");
        let part = signal.signal != bf.borrow()[0]["signal_l"][0][0];
        <BF_SIGNALS as BfSignalsExt>::insert(bf, index_, "signal_l", vec![vec![signal.signal]]);
        if part {
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

    use crate::test_funcs::test_funcs::*;

    static SIGNAL: LazyLock<CHANGE_SIGNAL> = LazyLock::new(|| CHANGE_SIGNAL::new());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(-1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        let mut a = vec![vec![Signal::new(1.0, 1.0)]; 2];
        a.reserve(1);
        a.push(vec![Signal::new(-1.0, 1.0)]);
        a
    });

    #[test]
    fn change_signal_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn change_signal_res_1() {
        test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn change_signal_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 3);
    }

    #[test]
    fn change_signal_coll_res_2() {
        test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 3);
    }

    #[test]
    fn change_signal_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC,
            &SIGNALS,
            vec![
                Signal { signal: 0.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
            ],
        );
    }
}
