#![allow(non_camel_case_types)]

use std::cmp::Ordering;

use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CHANGE_SRC {
    pub signal_short: f64,
    pub signal_long: f64,
    pub signal_hold: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl CHANGE_SRC {
    pub fn new(signal_short: f64, signal_long: f64, signal_hold: f64) -> Self {
        Self {
            signal_short,
            signal_long,
            signal_hold,
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
    pub fn set_signal_short(&mut self, signal_short: f64) {
        self.signal_short = signal_short;
    }
    pub fn set_signal_long(&mut self, signal_long: f64) {
        self.signal_long = signal_long;
    }
    pub fn set_signal_hold(&mut self, signal_hold: f64) {
        self.signal_hold = signal_hold;
    }
}

impl Default for CHANGE_SRC {
    fn default() -> Self {
        CHANGE_SRC::new(-1., 1., 0.)
    }
}

impl SignalReady for CHANGE_SRC {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(&self, src: &[Vec<f64>], _: &[Vec<Signal>]) -> BF_SIGNALS<'a> {
        <BF_SIGNALS as BfSignalsExt>::new([("src_l", vec![vec![src[src.len() - 1][0]]])])
    }
    fn signal_with_bf<'a>(
        &self,
        src: &[f64],
        _: &[Signal],
        bf: &BF_SIGNALS<'a>,
        index_: usize,
    ) -> Signal {
        let sr = *src.get(0).expect("src not found");
        let ord = sr.partial_cmp(&bf.borrow()[0]["src_l"][0][0]).unwrap();
        <BF_SIGNALS as BfSignalsExt>::insert(bf, index_, "src_l", vec![vec![sr]]);
        match ord {
            Ordering::Greater => Signal::new(self.signal_long, 1.0),
            Ordering::Less => Signal::new(self.signal_short, 1.0),
            Ordering::Equal => Signal::new(self.signal_hold, 1.0),
        }
    }
}

impl SignalReadyExt for CHANGE_SRC {}
