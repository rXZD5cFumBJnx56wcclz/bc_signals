use crate::ready::prelude::*;

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

impl Default for CONVERT {
    fn default() -> Self {
        CONVERT::new()
    }
}

impl SignalReady for CONVERT {
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
        src: &[f64],
        _: &[Signal],
        _: &BF_SIGNALS<'a>,
        _: usize,
    ) -> Signal {
        Signal::new(src[0], src[1])
    }
}

impl SignalReadyExt for CONVERT {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ready::test_funcs::test_funcs::*;

    static SIGNAL: LazyLock<CONVERT> = LazyLock::new(|| CONVERT::new());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 1.0]; 2]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| Default::default());

    #[test]
    fn convert_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn convert_signal_res_1() {
        test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn convert_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
    }

    #[test]
    fn convert_coll_res_2() {
        test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
    }

    #[test]
    fn convert_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC,
            &SIGNALS,
            vec![
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
            ],
        );
    }
}
