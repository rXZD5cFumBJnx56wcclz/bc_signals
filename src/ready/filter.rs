use crate::ready::prelude::*;

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
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
        }
    }
    pub fn set_window(
        &mut self,
        window: usize,
    ) {
        self.window = window;
    }
    pub fn set_mult_window_accuracy(
        &mut self,
        mult_window_accuracy: usize,
    ) {
        self.mult_window_accuracy = mult_window_accuracy;
    }
    pub fn set_add_window_accuracy(
        &mut self,
        add_window_accuracy: usize,
    ) {
        self.add_window_accuracy = add_window_accuracy;
    }
}

impl Default for FILTER {
    fn default() -> Self {
        FILTER::new()
    }
}

impl SignalReady for FILTER {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(
        &self,
        _: &[Vec<f64>],
        _: &[Vec<Signal>],
    ) -> BF_SIGNALS<'a> {
        Default::default()
    }
    fn signal_with_bf<'a>(
        &self,
        _: &[f64],
        signals: &[Signal],
        _: &BF_SIGNALS<'a>,
        _: usize,
    ) -> Signal {
        if signals.iter().all(|s| s == &signals[0]) {
            return signals[0];
        }
        Default::default()
    }
}

impl SignalReadyExt for FILTER {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ready::test_funcs::test_funcs::*;

    static SIGNAL: LazyLock<FILTER> = LazyLock::new(|| FILTER::new());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        vec![vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)], vec![Signal::new(1.0, 1.0); 2]]
    });

    #[test]
    fn filter_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn filter_signal_res_1() {
        test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn filter_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
    }

    #[test]
    fn filter_coll_res_2() {
        test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
    }

    #[test]
    fn filter_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC,
            &SIGNALS,
            vec![
                Signal { signal: 0.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
            ],
        );
    }
}
