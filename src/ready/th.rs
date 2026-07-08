use crate::ready::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TH {
    pub th_min: f64,
    pub th_max: f64,
    pub limit: f64,
    pub index_min: usize,
    pub index_max: usize,
    pub index_normal: usize,
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
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
            th_min,
            th_max,
            limit,
            index_min,
            index_max,
            index_normal,
            signal_hold: signal_hold,
            signal_short: signal_short,
            signal_long: signal_long,
            window: 1,
            mult_window_accuracy: 1,
            add_window_accuracy: 1,
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
    pub fn set_signal_hold(
        &mut self,
        signal_hold: f64,
    ) {
        self.signal_hold = signal_hold;
    }
    pub fn set_signal_short(
        &mut self,
        signal_short: f64,
    ) {
        self.signal_short = signal_short;
    }
    pub fn set_signal_long(
        &mut self,
        signal_long: f64,
    ) {
        self.signal_long = signal_long;
    }
    pub fn set_th_min(
        &mut self,
        th_min: f64,
    ) {
        self.th_min = th_min;
    }
    pub fn set_th_max(
        &mut self,
        th_max: f64,
    ) {
        self.th_max = th_max;
    }
    pub fn set_limit(
        &mut self,
        limit: f64,
    ) {
        self.limit = limit;
    }
    pub fn set_index_min(
        &mut self,
        index_min: usize,
    ) {
        self.index_min = index_min;
    }
    pub fn set_index_max(
        &mut self,
        index_max: usize,
    ) {
        self.index_max = index_max;
    }
    pub fn set_index_normal(
        &mut self,
        index_normal: usize,
    ) {
        self.index_normal = index_normal;
    }
}

impl Default for TH {
    fn default() -> Self {
        TH::new(0.03, 0.03, 0.15, 0, 0, 0, 0., -1., 1.)
    }
}

impl SignalReady for TH {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn bf<'a>(
        &self,
        src: &[Vec<f64>],
        _: &[Vec<Signal>],
    ) -> BF_SIGNALS<'a> {
        <BF_SIGNALS as BfSignalsExt>::new([("src_l", vec![src[src.len() - 1].to_vec()])])
    }
    fn signal_with_bf<'a>(
        &self,
        src: &[f64],
        _: &[Signal],
        bf: &BF_SIGNALS<'a>,
        index_: usize,
    ) -> Signal {
        let src_l = bf.borrow()[index_]["src_l"][0].clone();
        let perc_min = (src[self.index_normal] - src_l[self.index_min]) / src_l[self.index_normal];
        let perc_max = (src[self.index_normal] - src_l[self.index_max]) / src_l[self.index_normal];
        bf.roll_and_replace(-1, index_, "src_l", src.to_vec());
        let perc = perc_min.abs() + perc_max.abs();
        if perc >= self.th_min + self.th_max && perc <= self.limit * 2. {
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
    use bc_utils_lg::statics::prices::SRC;

    use crate::ready::test_funcs::test_funcs::*;

    static SIGNAL: LazyLock<TH> =
        LazyLock::new(|| TH::new(0.0001, 0.0001, 1.0, 1, 1, 1, 0., -1., 1.));
    const RES: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
    static SIGNALS: LazyLock<Vec<Vec<Signal>>> = LazyLock::new(|| {
        (0..SRC.len())
            .map(|_| vec![Signal::default()])
            .collect::<Vec<Vec<Signal>>>()
    });

    #[test]
    fn th_with_bf_res_1() {
        test_bf_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn th_signal_res_1() {
        test_f_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES);
    }

    #[test]
    fn th_coll_res_1() {
        test_coll_res_1(&*SIGNAL, &SRC, &SIGNALS, *RES, 30);
    }

    #[test]
    fn th_coll_res_2() {
        test_coll_res_2(&*SIGNAL, &SRC, &SIGNALS, 30);
    }

    #[test]
    fn th_coll_res_3() {
        test_coll_res_3(
            &*SIGNAL,
            &SRC,
            &SIGNALS,
            vec![
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 0.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 0.0, probability: 1.0 },
                Signal { signal: 0.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 0.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
                Signal { signal: -1.0, probability: 1.0 },
                Signal { signal: 1.0, probability: 1.0 },
            ],
        );
    }
}
