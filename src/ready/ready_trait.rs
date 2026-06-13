use std::{any::Any, cell::RefCell};

use bc_utils_lg::types::maps::MAP;

#[derive(Debug, PartialEq, Clone)]
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
    let w = signal_struct.w() - 1;
    let bf = signal_struct.bf(&src[..w], signals.get(..w).unwrap_or_default());
    src.iter()
        .enumerate()
        .map(|(i, v)| {
            if i < w {
                Signal::new(f64::NAN, 1.0)
            } else {
                signal_struct.signal_with_bf(v, signals.get(i).unwrap_or(&vec![]), &bf, 0)
            }
        })
        .collect()
}

pub trait SignalsReady: Any {
    fn w(&self) -> usize;
    fn bf(
        &self,
        src: &[Vec<f64>],
        signals: &[Vec<Signal>],
    ) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>;
    fn signal_with_bf(
        &self,
        src: &[f64],
        signals: &[Signal],
        bf: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        index_: usize,
    ) -> Signal;
    fn signal(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> Signal {
        let bf = self.bf(
            &src[src.len() - self.w()..src.len() - 1],
            &signals[src.len() - self.w()..src.len() - 1],
        );
        self.signal_with_bf(src.last().unwrap(), &signals[src.len() - 1], &bf, 0)
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
