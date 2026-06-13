use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FILTER {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl FILTER {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 1,
            add_window_accuracy: 1,
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

impl Default for FILTER {
    fn default() -> Self {
        FILTER::new()
    }
}

impl SignalsReady for FILTER {
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
        _: &[f64],
        signals: &[Signal],
        _: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        _: usize,
    ) -> Signal {
        if signals.iter().all(|s| s == &signals[0]) {
            return signals[0].clone();
        }
        Default::default()
    }
}

impl SignalsReadyExt for FILTER {}
