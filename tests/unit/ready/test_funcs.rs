use bc_signals::ready::ready_imports::*;

pub fn test_bf_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<Signal>], eq: Signal)
where
    T: SignalReady,
    T: SignalReadyExt,
{
    let len_sub_in = in_.len().checked_sub(1).unwrap_or_default();
    let len_sub_signals = signals.len().checked_sub(1).unwrap_or_default();
    let bf = settings_signal.bf(
        in_.get(..len_sub_in).unwrap_or_default(),
        signals.get(..len_sub_signals).unwrap_or_default(),
    );
    assert_eq!(
        settings_signal.signal_with_bf(
            in_.last().unwrap_or(&vec![0.0]),
            &signals.get(len_sub_signals).unwrap_or(&vec![]),
            &bf,
            0
        ),
        eq,
    );
}

pub fn test_f_res_1<T>(settings_signal: &T, in_: &[Vec<f64>], signals: &[Vec<Signal>], eq: Signal)
where
    T: SignalReady,
    T: SignalReadyExt,
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
    T: SignalReady,
    T: SignalReadyExt,
{
    assert_eq!(
        dbg!(
            settings_signal.signal_coll::<Vec<_>>(
                &in_.get(in_.len().checked_sub(len_elements).unwrap_or_default()..)
                    .unwrap_or_default(),
                signals,
            )
        )[len_elements - 1],
        eq,
    );
}

pub fn test_coll_res_2<T>(
    settings_signal: &T,
    in_: &[Vec<f64>],
    signals: &[Vec<Signal>],
    len_elements: usize,
) where
    T: SignalReadyExt,
{
    let in_ = &in_
        .get(in_.len().checked_sub(len_elements).unwrap_or_default()..)
        .unwrap_or_default();
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
    T: SignalReady,
    T: SignalReadyExt,
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
