use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CONVERT;

impl W for CONVERT {
    fn w(&self) -> usize {
        0
    }
}

impl SignalReady for CONVERT {
    fn init_bf(&self, _src: &[Vec<f64>], _signals: &[Vec<Signal>]) {}
    fn execute_bf(&self) {}
    fn signal(&self, src: &[f64], _signals: &[Signal]) -> Signal {
        Signal::new(src[0], src[1])
    }
}

impl SignalReadyExt for CONVERT {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::prelude_tests::prelude::*;

    static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 1.0]; 3]);
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| Default::default());

    #[test]
    fn convert_with_bf_res_1() {
        test_bf_res_1(&CONVERT, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn convert_coll_res_1() {
        test_coll_res_1(&CONVERT, &SRC, &SIGNALS, 1);
    }
}
