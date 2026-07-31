use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct ThParams {
    pub th_min: f64,
    pub th_max: f64,
    pub limit: f64,
    pub index_min: usize,
    pub index_max: usize,
    pub index_normal: usize,
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
}

impl Default for ThParams {
    fn default() -> Self {
        Self {
            th_min: 0.03,
            th_max: 0.03,
            limit: 0.15,
            index_min: 0,
            index_max: 0,
            index_normal: 0,
            signal_hold: 0.,
            signal_short: -1.,
            signal_long: 1.,
        }
    }
}

impl ThParams {
    pub fn new(
        th_min: f64,
        th_max: f64,
        limit: f64,
        index_min: usize,
        index_max: usize,
        index_normal: usize,
        signal_hold: f64,
        signal_short: f64,
        signal_long: f64,
    ) -> Self {
        Self {
            th_min,
            th_max,
            limit,
            index_min,
            index_max,
            index_normal,
            signal_hold,
            signal_short,
            signal_long,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ThBf {
    src_l: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TH {
    pub params: ThParams,
    bf: RefCell<ThBf>,
    bf_state: RefCell<ThBf>,
}

impl TH {
    pub fn new(
        th_min: f64,
        th_max: f64,
        limit: f64,
        index_min: usize,
        index_max: usize,
        index_normal: usize,
        signal_hold: f64,
        signal_short: f64,
        signal_long: f64,
    ) -> Self {
        Self {
            params: ThParams::new(
                th_min,
                th_max,
                limit,
                index_min,
                index_max,
                index_normal,
                signal_hold,
                signal_short,
                signal_long,
            ),
            ..Default::default()
        }
    }
}

impl W for TH {
    fn w(&self) -> usize {
        1
    }
}

impl SignalReady for TH {
    fn init_bf(&self, src: &[Vec<f64>], _signals: &[Vec<Signal>]) {
        self.bf.borrow_mut().src_l = src[src.len() - 1].to_vec();
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
    }
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
    fn signal(&self, src: &[f64], _signals: &[Signal]) -> Signal {
        let src_l = self.bf.borrow().src_l.clone();
        let perc_min = (src[self.params.index_normal] - src_l[self.params.index_min])
            / src_l[self.params.index_normal];
        let perc_max = (src[self.params.index_normal] - src_l[self.params.index_max])
            / src_l[self.params.index_normal];
        let perc = perc_min.abs() + perc_max.abs();
        self.bf_state.borrow_mut().src_l = src.to_vec();
        if perc >= self.params.th_min + self.params.th_max && perc <= self.params.limit * 2. {
            if perc_min > 0. {
                return Signal::new(1.0, 1.0);
            } else if perc_max < 0. {
                return Signal::new(-1.0, 1.0);
            }
        }
        Signal::default()
    }
}

impl SignalReadyExt for TH {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    static SIGNAL: LazyLock<fn() -> TH> =
        LazyLock::new(|| || TH::new(0.0001, 0.0001, 1.0, 1, 1, 1, 0., -1., 1.));
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        (0..SRC.len())
            .map(|_| vec![Signal::default()])
            .collect::<Vec<Vec<Signal>>>()
    });

    #[test]
    fn th_with_bf_res_1() {
        test_bf_res_1(&SIGNAL(), &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn th_coll_res_1() {
        test_coll_res_1(&SIGNAL(), &SRC, &SIGNALS, 10);
    }
}
