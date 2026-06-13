#![allow(non_camel_case_types)]

use crate::ready::ready_imports::*;
use bc_indicators::indicators::{osc_mult::OSC_MULT as IND, ready_imports::Indicator};

#[derive(Debug, PartialEq, Clone)]
pub struct OSC_MULT {
    pub th_short: f64,
    pub th_long: f64,
    pub max_value: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl OSC_MULT {
    pub fn new(th_short: f64, th_long: f64, max_value: f64) -> Self {
        Self {
            th_short,
            th_long,
            max_value,
            window: 0,
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
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
    pub fn set_th_short(&mut self, th_short: f64) {
        self.th_short = th_short;
    }
    pub fn set_th_long(&mut self, th_long: f64) {
        self.th_long = th_long;
    }
    pub fn set_max_value(&mut self, max_value: f64) {
        self.max_value = max_value;
    }
}

impl Default for OSC_MULT {
    fn default() -> Self {
        let ind = IND::default();
        OSC_MULT::new(ind.th_short, ind.th_long, ind.max_value)
    }
}

impl SignalsReady for OSC_MULT {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf(
        &self,
        _: &[Vec<f64>],
        _: &[Vec<Signal>],
    ) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>> {
        Default::default()
    }
    fn signal_with_bf(
        &self,
        src: &[f64],
        signals: &[Signal],
        _: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        _: usize,
    ) -> Signal {
        let mut signal = signals[0].clone();
        signal.probability = IND::new(self.th_short, self.th_long, self.max_value).ind(src);
        signal
    }
}

impl SignalsReadyExt for OSC_MULT {}
