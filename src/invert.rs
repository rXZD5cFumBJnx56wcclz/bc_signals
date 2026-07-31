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

impl W for INVERT {
    fn w(&self) -> usize {
        0
    }
}

impl SignalReady for INVERT {
    fn init_bf(&self, _src: &[Vec<f64>], _signals: &[Vec<Signal>]) {}
    fn execute_bf(&self) {}
    fn signal(&self, _src: &[f64], signals: &[Signal]) -> Signal {
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

    static SIGNAL: LazyLock<INVERT> = LazyLock::new(|| INVERT::default());
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(-1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> =
        LazyLock::new(|| vec![vec![Signal::new(1., 1.),]; 3]);

    #[test]
    fn invert_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &[], &SIGNALS, *RES);
    }

    #[test]
    fn invert_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &[], &SIGNALS, 1);
    }
}
