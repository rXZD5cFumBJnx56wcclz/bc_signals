use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct INVERT {
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
}

impl Default for INVERT {
    fn default() -> Self {
        Self {
            signal_hold: 0.,
            signal_short: -1.,
            signal_long: 1.,
        }
    }
}

impl INVERT {
    pub fn new(signal_hold: f64, signal_short: f64, signal_long: f64) -> Self {
        Self {
            signal_hold,
            signal_short,
            signal_long,
        }
    }
}

impl W for INVERT{
    fn w(&self) -> usize {
        0
    }
}

impl SignalReady for INVERT {
    fn init_bf(&self, _src: &[Vec<f64>], _signals: &[Vec<Signal>]) {}
    fn execute_bf(&self) {}
    fn signal_with_bf(&self, _src: &[f64], signals: &[Signal]) -> Signal {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;
    use crate::th::*;

    static SIGNAL: LazyLock<INVERT> = LazyLock::new(|| INVERT::default());
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(-1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        TH::new(0.0001, 0.0001, 1.0, 1, 1, 1, 0., -1., 1.)
            .signals_vec(&*SRC, &vec![])
            .into_iter()
            .map(|s| vec![s])
            .collect::<Vec<Vec<Signal>>>()
    });

    #[test]
    fn invert_with_bf_res_1() {
        test_bf_res_1(
            &*SIGNAL,
            &SRC.iter()
                .map(|v| v[1..].to_vec())
                .collect::<Vec<Vec<f64>>>(),
            &SIGNALS,
            *RES,
        );
    }

    #[test]
    fn invert_signal_res_1() {
        test_f_res_1(
            &*SIGNAL,
            &SRC.iter()
                .map(|v| v[1..].to_vec())
                .collect::<Vec<Vec<f64>>>(),
            &SIGNALS,
            *RES,
        );
    }

    #[test]
    fn invert_coll_res_1() {
        test_coll_res_1(
            &*SIGNAL,
            &SRC.iter()
                .map(|v| v[1..].to_vec())
                .collect::<Vec<Vec<f64>>>(),
            &SIGNALS,
            *RES,
            30,
        );
    }

    #[test]
    fn invert_coll_res_2() {
        test_coll_res_2(
            &*SIGNAL,
            &SRC.iter()
                .map(|v| v[1..].to_vec())
                .collect::<Vec<Vec<f64>>>(),
            &SIGNALS,
            30,
        );
    }

    #[test]
    fn invert_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC.iter()
                .map(|v| v[1..].to_vec())
                .collect::<Vec<Vec<f64>>>(),
            &SIGNALS,
            vec![
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -0.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -0.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -0.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -0.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: 1.0,
                    probability: 1.0,
                },
                Signal {
                    signal: -1.0,
                    probability: 1.0,
                },
            ],
        );
    }
}
