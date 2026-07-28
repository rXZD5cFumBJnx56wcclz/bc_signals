use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct REPEAT {
    pub value_signal: f64,
    pub value_probability: f64,
}

impl REPEAT {
    pub fn new(value_signal: f64, value_probability: f64) -> Self {
        Self {
            value_signal,
            value_probability,
        }
    }
}

impl W for REPEAT{
    fn w(&self) -> usize {
        0
    }
}

impl SignalReady for REPEAT {
    fn init_bf(&self, _src: &[Vec<f64>], _signals: &[Vec<Signal>]) {}
    fn execute_bf(&self) {}
    fn signal_with_bf(&self, _src: &[f64], _signals: &[Signal]) -> Signal {
        Signal::new(self.value_signal, self.value_probability)
    }
}

impl SignalReadyExt for REPEAT {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::prelude_tests::prelude::*;

    static SIGNAL: LazyLock<REPEAT> = LazyLock::new(|| REPEAT::default());
    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(0.0, 0.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        vec![
            vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
            vec![Signal::new(1.0, 1.0); 2],
        ]
    });

    #[test]
    fn repeat_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn repeat_signal_res_1() {
        test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn repeat_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 2);
    }

    #[test]
    fn repeat_coll_res_2() {
        test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 2);
    }

    #[test]
    fn repeat_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC,
            &SIGNALS,
            vec![
                Signal {
                    signal: 0.0,
                    probability: 0.0,
                },
                Signal {
                    signal: 0.0,
                    probability: 0.0,
                },
            ],
        );
    }
}
