use std::{any::Any, cell::RefCell};

use bc_utils_lg::types::maps::MAP;

fn signal_coll<C, T>(signal_struct: &T, src: &[Vec<f64>]) -> C
where
    C: FromIterator<f64>,
    T: TrainSignals,
    T: ?Sized,
{
    let bf = signal_struct.bf(&src[..signal_struct.get_window()
        * signal_struct.get_mult_window_accuracy()
        + signal_struct.get_add_window_accuracy()]);
    src.iter()
        .enumerate()
        .map(|(i, v)| {
            if i < signal_struct.get_window() * signal_struct.get_mult_window_accuracy()
                + signal_struct.get_add_window_accuracy()
            {
                f64::NAN
            } else {
                signal_struct.signal_with_bf(v, &bf, 0)
            }
        })
        .collect()
}

pub trait TrainSignals: Any {
    fn get_window(&self) -> usize;
    fn get_mult_window_accuracy(&self) -> usize;
    fn get_add_window_accuracy(&self) -> usize;
    fn bf(&self, src: &[Vec<f64>]) -> RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>;
    fn signal_with_bf(
        &self,
        src: &[f64],
        bf: &RefCell<Vec<MAP<&'static str, Vec<Vec<f64>>>>>,
        index_: usize,
    ) -> f64;
    fn signal(&self, src: &[Vec<f64>]) -> f64 {
        let bf = self.bf(&src[src.len()
            - (self.get_window() * self.get_mult_window_accuracy() + self.get_add_window_accuracy())
            - 1..src.len() - 1]);
        self.signal_with_bf(src.last().unwrap(), &bf, 0)
    }
    fn signals_vec(&self, src: &[Vec<f64>]) -> Vec<f64> {
        signal_coll(self, src)
    }
}

pub trait TrainSignalsExt: TrainSignals {
    fn signal_coll<C>(&self, src: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        signal_coll(self, src)
    }
}
