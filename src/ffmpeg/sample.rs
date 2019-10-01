use super::duration;
use super::frame;
use super::ErrorKind;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct SampleWindow {
    pub n: u32,
    pub start: u32,
    pub end: Option<u32>,
}

pub fn sample_timecodes(start: u32, end: u32, n: u32) -> Vec<u32> {
    let start_f = start as f32;
    let end_f = end as f32;
    let n_f = n as f32;
    (0..n)
        .map(|i| (start_f + (end_f - start_f) * (i as f32 / n_f)).floor() as u32)
        .collect::<Vec<u32>>()
}

type FrameHash<'a> = HashMap<u32, frame::Frame<'a>>;

pub fn sample_video<'a>(
    path_str: &'a String,
    window: &SampleWindow,
) -> Result<FrameHash<'a>, ErrorKind> {
    let d = duration(path_str).map(|dur_flt| dur_flt.floor() as u32)?;

    let par_iter = sample_timecodes(window.start, window.end.unwrap_or(d), window.n)
        .into_par_iter()
        .filter_map(|code| {
            let mut frm = frame::Frame::new(&path_str, code).ok()?;
            frm.read().and_then(|_| Ok((code, frm))).ok()
        });
    Ok(HashMap::from_par_iter(par_iter))
}
