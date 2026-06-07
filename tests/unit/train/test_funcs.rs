use bc_signals::train::ready_imports::*;
use bc_utils::nums::coll_nz;

pub fn test_bf_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], eq: f64)
where
    T: TrainSignals,
    T: TrainSignalsExt,
{
    let bf = settings_signal.bf(in_
        .into_iter()
        .cloned()
        .take(in_.len() - 1)
        .collect::<Vec<Vec<f64>>>()
        .as_slice());
    dbg!(&bf);
    assert_eq!(
        settings_signal.signal_with_bf(in_.last().unwrap(), &bf, 0),
        eq,
    );
}

pub fn test_f_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], eq: f64)
where
    T: TrainSignals,
    T: TrainSignalsExt,
{
    assert_eq!(settings_signal.signal(in_), eq,);
}

pub fn test_coll_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], eq: f64, len_elements: usize)
where
    T: TrainSignals,
    T: TrainSignalsExt,
{
    assert_eq!(
        dbg!(settings_signal.signal_coll::<Vec<_>>(&in_[in_.len() - len_elements..]))
            [len_elements - 1],
        eq,
    );
}

pub fn test_coll_res_2<T>(settings_signal: &T, in_: &[Vec<f64>], len_elements: usize)
where
    T: TrainSignalsExt,
{
    let in_ = &in_[in_.len() - len_elements..];
    assert_eq!(
        settings_signal.signal_coll::<Vec<_>>(in_)[len_elements - 1],
        settings_signal.signal(in_),
    );
}

pub fn test_coll_res_3<T>(settings_signal: &T, in_: &[Vec<f64>], eq: Vec<f64>)
where
    T: TrainSignals,
    T: TrainSignalsExt,
{
    assert_eq!(
        coll_nz::<Vec<f64>, f64, _>(&settings_signal.signal_coll::<Vec<_>>(&in_), 0.),
        coll_nz::<Vec<f64>, f64, _>(&eq, 0.),
    );
}
