use std::cmp::{Ordering::Equal, min_by_key};

use bc_utils::other::roll_slice1;

use crate::train::ready_imports::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MM {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
    pub index_min: usize,
    pub index_max: usize,
    pub min_distance: usize,
    pub max_distance: usize,
    pub tp_th: f64,
    pub tp_limit: f64,
}

impl MM {
    pub fn new(
        index_min: usize,
        index_max: usize,
        min_distance: usize,
        max_distance: usize,
        tp_th: f64,
        tp_limit: f64,
    ) -> Self {
        Self {
            window: max_distance,
            mult_window_accuracy: 1,
            add_window_accuracy: 0,
            index_min,
            index_max,
            min_distance,
            max_distance,
            tp_th,
            tp_limit,
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
    pub fn set_index_min(&mut self, index_min: usize) {
        self.index_min = index_min;
    }
    pub fn set_index_max(&mut self, index_max: usize) {
        self.index_max = index_max;
    }
    pub fn set_min_distance(&mut self, min_distance: usize) {
        self.min_distance = min_distance;
    }
    pub fn set_max_distance(&mut self, max_distance: usize) {
        self.max_distance = max_distance;
    }
    pub fn set_tp_th(&mut self, tp_th: f64) {
        self.tp_th = tp_th;
    }
    pub fn set_tp_limit(&mut self, tp_limit: f64) {
        self.tp_limit = tp_limit;
    }

}

impl Default for MM {
    fn default() -> Self {
        MM::new(0, 0, 10, 60, 0.03, 0.05)
    }
}

impl TrainSignals for MM {
    fn get_window(&self) -> usize {
        self.window
    }
    fn get_mult_window_accuracy(&self) -> usize {
        self.mult_window_accuracy
    }
    fn get_add_window_accuracy(&self) -> usize {
        self.add_window_accuracy
    }
    fn bf(&self, src: &[Vec<f64>]) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>> {
        RefCell::new(vec![MAP::from_iter([(
            "src_l_vec",
            src[src.len() - self.window * self.mult_window_accuracy - self.add_window_accuracy..]
                .to_vec(),
        )])])
    }
    fn signal_with_bf(
        &self,
        src: &[f64],
        bf: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        index_: usize,
    ) -> f64 {
        roll_slice1(
            bf.borrow_mut()
                .get_mut(index_)
                .unwrap()
                .get_mut("src_l_vec")
                .unwrap()
                .as_mut_slice(),
            &-1,
        );
        bf.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .get_mut("src_l_vec")
            .unwrap()
            .as_mut_slice()[self.window - 1] = src.to_vec();
        let bind = bf.borrow();
        let v = bind[index_]["src_l_vec"].iter().cloned().enumerate();
        let min_ = (
            v.clone()
                .min_by(|v1, v2| v1.1.partial_cmp(&v2.1).unwrap_or(Equal))
                .unwrap_or_default(),
            1.0,
        );
        let max_ = (
            v.clone()
                .max_by(|v1, v2| v1.1.partial_cmp(&v2.1).unwrap_or(Equal))
                .unwrap_or_default(),
            -1.0,
        );
        let percent =
            (max_.0.1[self.index_max] - min_.0.1[self.index_min]) / max_.0.1[self.index_max];
        if percent >= self.tp_th && percent <= self.tp_limit {
            if self.min_distance <= (min_.0.0.max(max_.0.0) - max_.0.0.min(min_.0.0)) {
                let res = min_by_key(max_, min_, |v1| v1.0.0);
                if res.0.0 == 0 {
                    return res.1;
                }
                return 0.0;
            }
            return 0.0;
        }
        0.0
    }
}

impl TrainSignalsExt for MM {}
