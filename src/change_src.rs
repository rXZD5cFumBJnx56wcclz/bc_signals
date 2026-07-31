#![allow(non_camel_case_types)]

use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeSrcParams {
    pub signal_short: f64,
    pub signal_long: f64,
    pub signal_hold: f64,
}

impl Default for ChangeSrcParams {
    fn default() -> Self {
        Self {
            signal_short: -1.,
            signal_long: 1.,
            signal_hold: 0.,
        }
    }
}

impl ChangeSrcParams {
    pub fn new(signal_short: f64, signal_long: f64, signal_hold: f64) -> Self {
        Self {
            signal_short,
            signal_long,
            signal_hold,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ChangeSrcBf {
    src_l: f64,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CHANGE_SRC {
    pub params: ChangeSrcParams,
    bf: RefCell<ChangeSrcBf>,
    bf_state: RefCell<ChangeSrcBf>,
}

impl CHANGE_SRC {
    pub fn new(signal_short: f64, signal_long: f64, signal_hold: f64) -> Self {
        Self {
            params: ChangeSrcParams::new(signal_short, signal_long, signal_hold),
            ..Default::default()
        }
    }
}

impl W for CHANGE_SRC {
    fn w(&self) -> usize {
        1
    }
}

impl SignalReady for CHANGE_SRC {
    fn init_bf(&self, src: &[Vec<f64>], _signals: &[Vec<Signal>]) {
        self.bf.borrow_mut().src_l = src[src.len() - 1][0];
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
    }
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
    fn signal(&self, src: &[f64], _signals: &[Signal]) -> Signal {
        let sr = *src.get(0).expect("src not found");
        let ord = sr.partial_cmp(&self.bf.borrow().src_l).unwrap();
        self.bf_state.borrow_mut().src_l = sr;
        match ord {
            Ordering::Greater => Signal::new(self.params.signal_long, 1.0),
            Ordering::Less => Signal::new(self.params.signal_short, 1.0),
            Ordering::Equal => Signal::new(self.params.signal_hold, 1.0),
        }
    }
}

impl SignalReadyExt for CHANGE_SRC {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::prelude_tests::prelude::*;

    static SIGNAL: LazyLock<fn() -> CHANGE_SRC> = LazyLock::new(|| || CHANGE_SRC::default());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0], vec![2.0], vec![3.0]]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        let mut a = vec![vec![Signal::new(1.0, 1.0)]; 2];
        a.reserve(1);
        a.push(vec![Signal::new(-1.0, 1.0)]);
        a
    });

    #[test]
    fn change_with_bf_res_1() {
        test_bf_res_1(&SIGNAL(), &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn change_src_coll_res_1() {
        test_coll_res_1(&SIGNAL(), &SRC, &SIGNALS, 1);
    }
}
