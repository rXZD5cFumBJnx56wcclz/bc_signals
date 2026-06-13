use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CHANGE {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl CHANGE {
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

impl Default for CHANGE {
    fn default() -> Self {
        CHANGE::new()
    }
}

impl SignalsReady for CHANGE {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf(
        &self,
        _: &[Vec<f64>],
        signals: &[Vec<Signal>],
    ) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>> {
        RefCell::new(vec![MAP::from_iter([(
            "signal_l",
            vec![vec![signals[signals.len() - 1][0].signal]],
        )])])
    }
    fn signal_with_bf(
        &self,
        _: &[f64],
        signals: &[Signal],
        bf: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        index_: usize,
    ) -> Signal {
        let signal = signals.get(0).expect("signal not found").clone();
        let part = signal.signal != bf.borrow()[0]["signal_l"][0][0];
        bf.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .insert("signal_l", vec![vec![signal.signal]]);
        if part { signal } else { Default::default() }
    }
}

impl SignalsReadyExt for CHANGE {}
