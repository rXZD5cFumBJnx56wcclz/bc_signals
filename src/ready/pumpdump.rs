use bc_utils::other::roll_slice1;

use crate::ready::ready_imports::*;

#[derive(Debug, PartialEq, Clone)]
pub struct PUMPDUMP {
    pub th_min: f64,
    pub th_max: f64,
    pub limit: f64,
    pub index_min: usize,
    pub index_max: usize,
    pub index_normal: usize,
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl PUMPDUMP {
    pub fn new(
        th_min: f64,
        th_max: f64,
        limit: f64,
        index_min: usize,
        index_max: usize,
        index_normal: usize,
    ) -> Self {
        Self {
            th_min,
            th_max,
            limit,
            index_min,
            index_max,
            index_normal,
            signal_hold: 0.0,
            signal_short: -1.0,
            signal_long: 1.0, 
            window: 1,
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
    pub fn set_signal_hold(&mut self, signal_hold: f64) {
        self.signal_hold = signal_hold;
    }
    pub fn set_signal_short(&mut self, signal_short: f64) {
        self.signal_short = signal_short;
    }
    pub fn set_signal_long(&mut self, signal_long: f64) {
        self.signal_long = signal_long;
    }
    pub fn set_index_min(&mut self, index_min: usize) {
        self.index_min = index_min;
    }
    pub fn set_index_max(&mut self, index_max: usize) {
        self.index_max = index_max;
    }
    pub fn set_index_normal(&mut self, index_normal: usize) {
        self.index_normal = index_normal;
    }
    pub fn set_th_min(&mut self, th_min: f64) {
        self.th_min = th_min;
    }
    pub fn set_th_max(&mut self, th_max: f64) {
        self.th_max = th_max;
    }
}

impl Default for PUMPDUMP {
    fn default() -> Self {
        PUMPDUMP::new(0.03, 0.03, 0.15, 0, 0, 0)
    }
}

impl SignalsReady for PUMPDUMP {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf(
        &self,
        src: &[Vec<f64>],
        _: &[Vec<Signal>],
    ) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>> {
        RefCell::new(vec![MAP::from_iter([(
            "src_l",
            vec![src[src.len() - 1].to_vec()],
        )])])
    }
    fn signal_with_bf(
        &self,
        src: &[f64],
        _: &[Signal],
        bf: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        index_: usize,
    ) -> Signal {
        let src_l = bf.borrow()[index_]["src_l"][0].clone();
        let perc_min = (src[self.index_normal] - src_l[self.index_min]) / src_l[self.index_normal];
        let perc_max = (src[self.index_normal] - src_l[self.index_max]) / src_l[self.index_normal];
        roll_slice1(
            bf.borrow_mut()
                .get_mut(index_)
                .unwrap()
                .get_mut("src_l")
                .unwrap(),
            &-1,
        );
        bf.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .insert("src_l", vec![src.to_vec()]);
        if perc_min.abs() >= self.th_min && perc_max.abs() >= self.th_max {
            if perc_min > 0. {
                return Signal::new(1.0, 1.0);
            } else if perc_max < 0. {
                return Signal::new(-1.0, 1.0);
            }
        }
        Signal::default()
    }
}

impl SignalsReadyExt for PUMPDUMP {}
