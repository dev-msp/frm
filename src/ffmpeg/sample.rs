use super::duration;
use super::frame;
use super::ErrorKind;
use rayon::prelude::*;

fn sample_timecodes(duration: u32, n: u32) -> Vec<u32> {
    let n_f = n as f32;
    let dur_f = duration as f32;
    (0..n)
        .map(|i| (dur_f * (i as f32 / n_f)).floor() as u32)
        .collect::<Vec<u32>>()
}

pub fn sample_video(path_str: String, n: u32) -> Result<(), ErrorKind> {
    let d = duration(&path_str).map(|dur_flt| dur_flt.floor() as u32)?;
    sample_timecodes(d, n)
        .into_par_iter()
        .enumerate()
        .map(|(i, code)| {
            let out_path = format!("output_{}.jpeg", i);
            frame::Frame::new(&path_str, code).and_then(|frm| frm.write(out_path))
        })
        .reduce(|| Ok(()), |acc, x| acc.and(x))
}
