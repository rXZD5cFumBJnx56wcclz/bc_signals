use bc_signals::ready::ready_imports::*;
// use bc_utils::nums::coll_nz;

pub fn test_bf_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<Signal>], eq: Signal)
where
    T: SignalsReady,
    T: SignalsReadyExt,
{
    let bf = settings_signal.bf(
        in_.into_iter()
            .cloned()
            .take(in_.len() - 1)
            .collect::<Vec<Vec<f64>>>()
            .as_slice(),
        signals,
    );
    dbg!(&bf, signals.len());
    assert_eq!(
        settings_signal.signal_with_bf(
            in_.last().unwrap(),
            &signals.get(in_.len() - 1).unwrap_or(&vec![]),
            &bf,
            0
        ),
        eq,
    );
}

pub fn test_f_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<Signal>], eq: Signal)
where
    T: SignalsReady,
    T: SignalsReadyExt,
{
    assert_eq!(settings_signal.signal(in_, signals,), eq,);
}

pub fn test_coll_res_1<T>(
    settings_signal: &T,
    in_: &[Vec<f64>],
    signals: &[Vec<Signal>],
    eq: Signal,
    len_elements: usize,
) where
    T: SignalsReady,
    T: SignalsReadyExt,
{
    assert_eq!(
        dbg!(settings_signal.signal_coll::<Vec<_>>(&in_[in_.len() - len_elements..], signals,))
            [len_elements - 1],
        eq,
    );
}

pub fn test_coll_res_2<T>(
    settings_signal: &T,
    in_: &[Vec<f64>],
    signals: &[Vec<Signal>],
    len_elements: usize,
) where
    T: SignalsReadyExt,
{
    let in_ = &in_[in_.len() - len_elements..];
    assert_eq!(
        settings_signal.signal_coll::<Vec<_>>(in_, signals,)[len_elements - 1],
        settings_signal.signal(in_, signals,),
    );
}

pub fn test_coll_res_3<T>(
    settings_signal: &T,
    in_: &[Vec<f64>],
    signals: &[Vec<Signal>],
    eq: Vec<Signal>,
) where
    T: SignalsReady,
    T: SignalsReadyExt,
{
    assert_eq!(
        settings_signal
            .signal_coll::<Vec<_>>(&in_, signals,)
            .into_iter()
            .filter(|v| !v.signal.is_nan())
            .collect::<Vec<Signal>>(),
        eq,
    );
}
