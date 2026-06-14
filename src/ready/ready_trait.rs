use std::any::Any;

use crate::def_impl::BF_SIGNALS;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Signal {
    pub signal: f64,
    pub probability: f64,
}

impl Signal {
    pub fn new(signal: f64, probability: f64) -> Self {
        Self {
            signal,
            probability,
        }
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new(0., 1.)
    }
}

fn signal_coll<C, T>(signal_struct: &T, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> C
where
    C: FromIterator<Signal>,
    T: SignalsReady,
    T: ?Sized,
{
    let w = signal_struct.w().checked_sub(1).unwrap_or_default();
    let bf = signal_struct.bf(
        &src.get(..w).unwrap_or_default(),
        signals.get(..w).unwrap_or_default(),
    );
    let map_func = |i: usize, src: &[f64], s: &[Signal]| {
        if i < w {
            Signal::new(f64::NAN, 1.0)
        } else {
            signal_struct.signal_with_bf(src, s, &bf, 0)
        }
    };
    match (src.is_empty(), signals.is_empty()) {
        (false, false) | (true, true) => src
            .iter()
            .zip(signals)
            .enumerate()
            .map(|(i, (src, s))| map_func(i, src, s))
            .collect(),
        (true, false) => signals
            .iter()
            .enumerate()
            .map(|(i, s)| map_func(i, Default::default(), s))
            .collect(),
        (false, true) => src
            .iter()
            .enumerate()
            .map(|(i, sr)| map_func(i, sr, Default::default()))
            .collect(),
    }
}

pub trait SignalsReady: Any {
    fn w(&self) -> usize;
    fn bf<'a>(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> BF_SIGNALS<'a>;
    fn signal_with_bf<'a>(
        &self,
        src: &[f64],
        signals: &[Signal],
        bf: &BF_SIGNALS<'a>,
        index_: usize,
    ) -> Signal;
    fn signal(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> Signal {
        let len_sub_one_signals = signals.len().checked_sub(1).unwrap_or_default();
        let bf;
        if self.w() != 0 {
            bf = self.bf(
                &src[src.len().checked_sub(self.w()).unwrap_or_default()
                    ..src.len().checked_sub(1).unwrap_or_default()],
                &signals
                    [signals.len().checked_sub(self.w()).unwrap_or_default()..len_sub_one_signals],
            );
        } else {
            bf = Default::default();
        }
        self.signal_with_bf(
            src.last().unwrap_or(&vec![]),
            signals.last().unwrap_or(&vec![Signal::default()]),
            &bf,
            0,
        )
    }
    fn signals_vec(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> Vec<Signal> {
        signal_coll(self, src, signals)
    }
}

pub trait SignalsReadyExt: SignalsReady {
    fn signal_coll<C>(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> C
    where
        C: FromIterator<Signal>,
    {
        signal_coll(self, src, signals)
    }
}
