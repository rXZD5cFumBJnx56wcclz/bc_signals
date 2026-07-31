use std::any::Any;

use bc_utils_lg::{structs::signals::Signal, traits::w::W};
use dyn_clone::DynClone;

fn signal_coll<C, T>(signal_struct: &T, src: &[Vec<f64>], signals: &[Vec<Signal>]) -> C
where
    C: FromIterator<Signal>,
    T: SignalReady,
    T: ?Sized,
{
    let map_func = |src: &[f64], s: &[Signal]| {
        let bind = signal_struct.signal(src, s);
        signal_struct.execute_bf();
        bind
    };
    match (src.is_empty(), signals.is_empty()) {
        (false, false) | (true, true) => src
            .iter()
            .zip(signals)
            .map(|(src, s)| map_func(src, s))
            .collect(),
        (true, false) => signals
            .iter()
            .map(|s| map_func(Default::default(), s))
            .collect(),
        (false, true) => src
            .iter()
            .map(|src| map_func(src, Default::default()))
            .collect(),
    }
}

pub trait SignalReady: Any + W + DynClone {
    fn init_bf(&self, src: &[Vec<f64>], signals: &[Vec<Signal>]);
    fn execute_bf(&self);
    fn signal(&self, src: &[f64], signals: &[Signal]) -> Signal;
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
