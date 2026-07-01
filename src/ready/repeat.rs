use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct REPEAT {
    pub value_signal: f64,
    pub value_probability: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl REPEAT {
    pub fn new(value_signal: f64, value_probability: f64) -> Self {
        Self {
            value_signal,
            value_probability,
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
    pub fn set_value_signal(&mut self, value_signal: f64) {
        self.value_signal = value_signal;
    }
    pub fn set_value_probability(&mut self, value_probability: f64) {
        self.value_probability = value_probability;
    }
}

impl Default for REPEAT {
    fn default() -> Self {
        REPEAT::new(0.0, 1.0)
    }
}

impl SignalReady for REPEAT {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(&self, _: &[Vec<f64>], _: &[Vec<Signal>]) -> BF_SIGNALS<'a> {
        Default::default()
    }
    fn signal_with_bf<'a>(&self, _: &[f64], _: &[Signal], _: &BF_SIGNALS<'a>, _: usize) -> Signal {
        Signal::new(self.value_signal, self.value_probability)
    }
}

impl SignalReadyExt for REPEAT {}
