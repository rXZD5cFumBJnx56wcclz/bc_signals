use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CONVERT {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl CONVERT {
    pub fn new() -> Self {
        Self {
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
}

impl Default for CONVERT {
    fn default() -> Self {
        CONVERT::new()
    }
}

impl SignalsReady for CONVERT {
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
        _: &[Signal],
        _: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        _: usize,
    ) -> Signal {
        Signal::new(src[0], src[1])
    }
}

impl SignalsReadyExt for CONVERT {}
