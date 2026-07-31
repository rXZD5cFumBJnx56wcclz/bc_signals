#[cfg(test)]
pub mod test_funcs {
    use pretty_assertions::assert_eq as assert_eq_pr;

    use crate::prelude::*;

    pub fn test_bf_res_1<T>(
        settings_signal: &T,
        in_: &[Vec<f64>],
        signals: &[Vec<Signal>],
        eq: Signal,
    ) where
        T: SignalReady,
        T: SignalReadyExt,
    {
        settings_signal.init_bf(
            in_.get(..in_.len().checked_sub(1).unwrap_or_default())
                .unwrap_or_default(),
            signals
                .get(..signals.len().checked_sub(1).unwrap_or_default())
                .unwrap_or_default(),
        );
        assert_eq_pr!(
            settings_signal.signal(
                &in_.last().cloned().unwrap_or_default(),
                &signals.last().cloned().unwrap_or_default()
            ),
            eq,
        );
    }

    pub fn test_coll_res_1<T>(
        settings_signal: &T,
        in_: &[Vec<f64>],
        signals: &[Vec<Signal>],
        interval_len: usize,
    ) where
        T: SignalReady,
        T: Clone,
        T: SignalReadyExt,
    {
        let sign_vec = settings_signal.clone();
        let len_src = in_.len().checked_sub(interval_len).unwrap_or_default();
        let len_signals = signals.len().checked_sub(interval_len).unwrap_or_default();
        sign_vec.init_bf(
            in_.get(..len_src).unwrap_or_default(),
            signals.get(..len_signals).unwrap_or_default(),
        );
        let sign_value = settings_signal.clone();
        sign_value.init_bf(
            in_.get(..in_.len().checked_sub(1).unwrap_or_default())
                .unwrap_or_default(),
            signals
                .get(..signals.len().checked_sub(1).unwrap_or_default())
                .unwrap_or_default(),
        );
        assert_eq_pr!(
            sign_vec
                .signals_vec(
                    in_.get(len_src..).unwrap_or_default(),
                    signals.get(len_signals..).unwrap_or_default(),
                )
                .last()
                .copied()
                .unwrap(),
            sign_value.signal(
                &in_.last().cloned().unwrap_or_default(),
                &signals.last().cloned().unwrap_or_default()
            ),
        );
    }
}
