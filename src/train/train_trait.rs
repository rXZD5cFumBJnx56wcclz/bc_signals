use std::{any::Any, cell::RefCell};

use bc_utils_lg::types::maps::MAP;

fn signal_coll<C, T>(signal_struct: &T, src: &[Vec<f64>], signals: &[Vec<f64>]) -> C
where
    C: FromIterator<f64>,
    T: SignalsTrain,
    T: ?Sized,
{
    let w = signal_struct.w() - 1;
    let bf = signal_struct.bf(&src[..w], &signals[..w]);
    src.iter()
        .zip(signals)
        .skip(w)
        .map(|(v, signal)| signal_struct.signal_with_bf(v, signal, &bf, 0))
        .chain(std::iter::repeat(f64::NAN).take(w))
        .collect()
}

pub trait SignalsTrain: Any {
    fn w(&self) -> usize;
    fn bf(
        &self,
        src: &[Vec<f64>],
        signals: &[Vec<f64>],
    ) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>;
    fn signal_with_bf(
        &self,
        src: &[f64],
        signals: &[f64],
        bf: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        index_: usize,
    ) -> f64;
    fn signal(&self, src: &[Vec<f64>], signals: &[Vec<f64>]) -> f64 {
        let bf = self.bf(
            &src[src.len() - self.w()..src.len() - 1],
            &signals[src.len() - self.w()..src.len() - 1],
        );
        self.signal_with_bf(src.last().unwrap(), &signals[src.len() - 1], &bf, 0)
    }
    fn signals_vec(&self, src: &[Vec<f64>], signals: &[Vec<f64>]) -> Vec<f64> {
        signal_coll(self, src, signals)
    }
}

pub trait SignalsTrainExt: SignalsTrain {
    fn signal_coll<C>(&self, src: &[Vec<f64>], signals: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        signal_coll(self, src, signals)
    }
}
