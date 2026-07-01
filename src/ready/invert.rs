use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct INVERT {
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl INVERT {
    pub fn new(signal_hold: f64, signal_short: f64, signal_long: f64) -> Self {
        Self {
            signal_hold: signal_hold,
            signal_short: signal_short,
            signal_long: signal_long,
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
    pub fn set_signal_hold(&mut self, signal_hold: f64) {
        self.signal_hold = signal_hold;
    }
    pub fn set_signal_short(&mut self, signal_short: f64) {
        self.signal_short = signal_short;
    }
    pub fn set_signal_long(&mut self, signal_long: f64) {
        self.signal_long = signal_long;
    }
}

impl Default for INVERT {
    fn default() -> Self {
        INVERT::new(0., -1., 1.)
    }
}

impl SignalReady for INVERT {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(&self, _: &[Vec<f64>], _: &[Vec<Signal>]) -> BF_SIGNALS<'a> {
        Default::default()
    }
    fn signal_with_bf<'a>(
        &self,
        _: &[f64],
        signals: &[Signal],
        _: &BF_SIGNALS<'a>,
        _: usize,
    ) -> Signal {
        let mut signal = *signals.get(0).expect("signal not found");
        if signal.signal == self.signal_short {
            signal.signal = self.signal_long;
        } else if signal.signal == self.signal_long {
            signal.signal = self.signal_short;
        }
        signal
    }
}

impl SignalReadyExt for INVERT {}
