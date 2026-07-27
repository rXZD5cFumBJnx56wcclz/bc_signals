use std::any::Any;

use bc_utils_lg::structs::signals::Signal;
use dyn_clone::DynClone;

fn signal_coll<C, T>(signal_struct: &T, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> C
where
    C: FromIterator<Signal>,
    T: SignalReady,
    T: ?Sized,
{
    let w = signal_struct.w().checked_sub(1).unwrap_or_default();
    signal_struct.init_bf(
        &src.get(..w).unwrap_or_default(),
        signals.get(..w).unwrap_or_default(),
    );
    let map_func = |i: usize, src: &[f64], s: &[Signal]| {
        if i < w {
            Signal::new(f64::NAN, 1.0)
        } else {
            let bind = signal_struct.signal_with_bf(src, s);
            signal_struct.execute_bf();
            bind
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

pub trait SignalReady: Any + DynClone {
    fn w(&self) -> usize;
    fn init_bf(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]);
    fn execute_bf(&self);
    fn signal_with_bf(&self, src: &[f64], signals: &[Signal]) -> Signal;
    fn signal(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> Signal {
        let len_sub_one_signals = signals.len().checked_sub(1).unwrap_or_default();
        if self.w() != 0 {
            self.init_bf(
                &src[src.len().checked_sub(self.w()).unwrap_or_default()
                    ..src.len().checked_sub(1).unwrap_or_default()],
                &signals
                    [signals.len().checked_sub(self.w()).unwrap_or_default()..len_sub_one_signals],
            );
        }
        self.signal_with_bf(
            src.last().unwrap_or(&vec![]),
            signals.last().unwrap_or(&vec![Signal::default()]),
        )
    }
    fn signals_vec(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> Vec<Signal> {
        signal_coll(self, src, signals)
    }
}

dyn_clone::clone_trait_object!(SignalReady);

pub trait SignalReadyExt: SignalReady {
    fn signal_coll<C>(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> C
    where
        C: FromIterator<Signal>,
    {
        signal_coll(self, src, signals)
    }
}
