#![allow(non_camel_case_types)]

use crate::ready::ready_imports::*;

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
    pub fn set_window(&mut self, window: usize) {
        self.window = window;
    }
    pub fn set_mult_window_accuracy(&mut self, mult_window_accuracy: usize) {
        self.mult_window_accuracy = mult_window_accuracy;
    }
    pub fn set_add_window_accuracy(&mut self, add_window_accuracy: usize) {
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
    fn bf<'a>(&self, _: &[Vec<f64>], signals: &[Vec<Signal>]) -> BF_SIGNALS<'a> {
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
        if part { signal } else { Default::default() }
    }
}

impl SignalReadyExt for CHANGE_SIGNAL {}
