use bc_signals::train::ready_imports::*;
use bc_utils::nums::coll_nz;

pub fn test_bf_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<f64>], eq: f64)
where
    T: SignalsTrain,
    T: SignalsTrainExt,
{
    let bf = settings_signal.bf(
        in_.into_iter()
            .cloned()
            .take(in_.len() - 1)
            .collect::<Vec<Vec<f64>>>()
            .as_slice(),
        signals,
    );
    assert_eq!(
        settings_signal.signal_with_bf(in_.last().unwrap(), &signals[in_.len() - 1], &bf, 0),
        eq,
    );
}

pub fn test_f_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<f64>], eq: f64)
where
    T: SignalsTrain,
    T: SignalsTrainExt,
{
    assert_eq!(settings_signal.signal(in_, signals,), eq,);
}

pub fn test_coll_res_1<T>(
    settings_signal: &T,
    in_: &[Vec<f64>],
    signals: &[Vec<f64>],
    eq: f64,
    len_elements: usize,
) where
    T: SignalsTrain,
    T: SignalsTrainExt,
{
    assert_eq!(
        dbg!(settings_signal.signal_coll::<Vec<_>>(&in_[in_.len() - len_elements..], signals,))
            [len_elements - 1 - settings_signal.w()],
        eq,
    );
}

pub fn test_coll_res_2<T>(
    settings_signal: &T,
    in_: &[Vec<f64>],
    signals: &[Vec<f64>],
    len_elements: usize,
) where
    T: SignalsTrainExt,
{
    let in_ = &in_[in_.len() - len_elements..];
    assert_eq!(
        settings_signal.signal_coll::<Vec<_>>(in_, signals,)
            [len_elements - 1 - settings_signal.w()],
        settings_signal.signal(in_, signals,),
    );
}

pub fn test_coll_res_3<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<f64>], eq: Vec<f64>)
where
    T: SignalsTrain,
    T: SignalsTrainExt,
{
    assert_eq!(
        coll_nz::<Vec<f64>, f64, _>(&settings_signal.signal_coll::<Vec<_>>(&in_, signals,), 0.),
        coll_nz::<Vec<f64>, f64, _>(&eq, 0.),
    );
}
