use super::duration;
use super::frame;
use super::ErrorKind;
use rayon::prelude::*;

pub struct SampleWindow {
    pub start: u32,
    pub end: Option<u32>,
    pub n: u32,
}

fn sample_timecodes(start: u32, end: u32, n: u32) -> Vec<u32> {
    let start_f = start as f32;
    let end_f = end as f32;
    let n_f = n as f32;
    (0..n)
        .map(|i| (start_f + (end_f - start_f) * (i as f32 / n_f)).floor() as u32)
        .collect::<Vec<u32>>()
}

pub fn sample_video(path_str: &String, window: &SampleWindow) -> Result<(), ErrorKind> {
    let d = duration(path_str).map(|dur_flt| dur_flt.floor() as u32)?;

    sample_timecodes(window.start, window.end.unwrap_or(d), window.n)
        .into_par_iter()
        .map(|code| {
            let out_path = format!("output_{}.jpeg", code);
            frame::Frame::new(&path_str, code).and_then(|mut frm| frm.write_file(out_path))
        })
        .reduce(|| Ok(()), |acc, x| acc.and(x))
}
