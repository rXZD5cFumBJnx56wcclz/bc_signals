#![allow(non_camel_case_types)]

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct COPY {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl COPY {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
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

impl Default for COPY {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalReady for COPY {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(
        &self,
        _: &[Vec<f64>],
        _: &[Vec<Signal>],
    ) -> BF_SIGNALS<'a> {
        Default::default()
    }
    fn signal_with_bf<'a>(
        &self,
        _: &[f64],
        signals: &[Signal],
        _: &BF_SIGNALS<'a>,
        _: usize,
    ) -> Signal {
        signals[0]
    }
}

impl SignalReadyExt for COPY {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_funcs::test_funcs::*;

    static SIGNAL: LazyLock<COPY> = LazyLock::new(|| COPY::default());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.7333333333333333,]; 2]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> =
        LazyLock::new(|| vec![vec![Signal::new(1.0, 1.0)]; 2]);

    #[test]
    fn copy_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn copy_signal_res_1() {
        test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn copy_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
    }

    #[test]
    fn copy_coll_res_2() {
        test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
    }

    #[test]
    fn copy_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC,
            &SIGNALS,
            vec![
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
            ],
        );
    }
}
