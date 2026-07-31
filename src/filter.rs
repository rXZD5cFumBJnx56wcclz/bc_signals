use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct FILTER;

impl W for FILTER {
    fn w(&self) -> usize {
        0
    }
}

impl SignalReady for FILTER {
    fn init_bf(&self, _src: &[Vec<f64>], _signals: &[Vec<Signal>]) {}
    fn execute_bf(&self) {}
    fn signal(&self, _src: &[f64], signals: &[Signal]) -> Signal {
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

    use crate::prelude_tests::prelude::*;

    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(0.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        vec![
            vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
            vec![Signal::new(1.0, 1.0); 2],
            vec![Signal::new(-1.0, 1.0), Signal::new(1.0, 1.0)],
        ]
    });

    #[test]
    fn filter_with_bf_res_1() {
        test_bf_res_1(&FILTER, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn filter_coll_res_1() {
        test_coll_res_1(&FILTER, &SRC, &SIGNALS, 1);
    }
}
