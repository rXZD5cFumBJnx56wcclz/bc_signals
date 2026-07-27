#![allow(non_camel_case_types)]

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct SET_PROBABILITY;

impl SignalReady for SET_PROBABILITY {
    fn w(&self) -> usize {
        0
    }
    fn init_bf(&self, _src: &[Vec<f64>], _signals: &[Vec<Signal>]) {}
    fn execute_bf(&self) {}
    fn signal_with_bf(&self, src: &[f64], signals: &[Signal]) -> Signal {
        let mut signal = signals[0];
        signal.probability = src[0];
        signal
    }
}

impl SignalReadyExt for SET_PROBABILITY {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::prelude_tests::prelude::*;

    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.7333333333333333,]; 2]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 0.7333333333333333));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> =
        LazyLock::new(|| vec![vec![Signal::new(1.0, 1.0)]; 2]);

    #[test]
    fn set_probability_with_bf_res_1() {
        test_bf_res_1(&SET_PROBABILITY, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn set_probability_signal_res_1() {
        test_f_res_1(&SET_PROBABILITY, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn set_probability_coll_res_1() {
        test_coll_res_1(&SET_PROBABILITY, &SRC, &SIGNALS, *RES, 2);
    }

    #[test]
    fn set_probability_coll_res_2() {
        test_coll_res_2(&SET_PROBABILITY, &SRC, &SIGNALS, 2);
    }

    #[test]
    fn set_probability_coll_res_3() {
        test_coll_res_3(
            &SET_PROBABILITY,
            &SRC,
            &SIGNALS,
            vec![
                Signal {
                    signal: 1.0,
                    probability: 0.7333333333333333,
                },
                Signal {
                    signal: 1.0,
                    probability: 0.7333333333333333,
                },
            ],
        );
    }
}
