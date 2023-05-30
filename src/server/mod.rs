use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};
use serde::Deserialize;
use serde_json::{json, Value};
use thiserror::Error;
use warp::{self, filters::BoxedFilter, path, Filter, Rejection, Reply};

use std::collections::HashMap;
use std::sync::Arc;

use crate::ffmpeg::ErrorKind;

use super::ffmpeg::{duration, frame};

#[allow(dead_code)]
pub struct SampleWindow {
    pub n: u32,
    pub start: u32,
    pub end: Option<u32>,
}

#[allow(dead_code)]
pub fn sample_timecodes(start: u32, end: u32, n: u32) -> Vec<(u32, u32)> {
    let start_f = start as f32;
    let end_f = end as f32;
    let n_f = n.min(end - start) as f32;
    (1..n + 1)
        .map(|i| {
            (
                (start_f + (end_f - start_f) * ((i - 1) as f32 / n_f)).floor() as u32,
                (start_f + (end_f - start_f) * (i as f32 / n_f)).floor() as u32,
            )
        })
        .collect::<Vec<(u32, u32)>>()
}

#[allow(dead_code)]
type FrameHash = HashMap<u32, frame::Frame>;

fn render_frame(file_str: String, i: u32) -> impl Reply {
    frame::Frame::new(file_str.as_ref(), i)
        .and_then(|frm| frm.write())
        .map(|r| warp::reply::with_header(r, "Content-Type", "image/jpeg"))
        .unwrap()
}

pub struct FrameServer {
    file: String,
    pool: Arc<ThreadPool>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error building server: {0}")]
    ServerInit(#[from] ThreadPoolBuildError),

    #[allow(dead_code)]
    #[error("unhandled: {0}")]
    Unhandled(String),
}

impl Error {
    #[allow(dead_code)]
    pub fn unhandled(s: impl Into<String>) -> Self {
        Self::Unhandled(s.into())
    }
}

fn format_time(n: u32) -> String {
    let h = n / 3600;
    let m = (n - 3600 * h) / 60;
    let s = n - 3600 * h - 60 * m;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

impl FrameServer {
    pub fn new(file: String, num_threads: usize) -> Result<FrameServer, Error> {
        let pool = ThreadPoolBuilder::default()
            .num_threads(num_threads)
            .build()?;
        Ok(FrameServer {
            file,
            pool: Arc::new(pool),
        })
    }

    pub fn serve(self) {
        let root = self
            .context_route()
            .or(self.data_route())
            .or(self.image_route());
        warp::serve(root).run(([127, 0, 0, 1], 3030));
    }

    fn image_route(&self) -> BoxedFilter<(impl Reply,)> {
        let file = self.file.clone();
        let pool = self.pool.clone();

        let render_frame = move |i| pool.install(|| render_frame(file.clone(), i));

        path("frame")
            .and(warp::get2())
            .and(path::param())
            .map(render_frame)
            .boxed()
    }

    fn context_route(&self) -> BoxedFilter<(impl Reply,)> {
        let file = self.file.clone();
        path("context")
            .and(path::end())
            .and_then(move || -> Result<String, Rejection> {
                match duration(&file) {
                    Ok(d) => Ok(json!({ "duration": d as u32 }).to_string()),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            })
            .boxed()
    }

    fn data_route(&self) -> BoxedFilter<(impl Reply,)> {
        let from = path::end()
            .map(|| <Option<u32>>::None)
            .or(path("from").and(path::param()).map(Some))
            .unify()
            .map(|o: Option<u32>| match o {
                Some(0) | None => 60,
                Some(n) => n,
            });

        let file = self.file.clone();
        let to = path("to")
            .and(path::param().map(Some))
            .or(path::end().map(|| None))
            .unify()
            .map(move |n: Option<i32>| {
                let n = n.unwrap_or(0);
                if n <= 0 {
                    duration(&file).map(|d| ((d as i32) - n) as u32)
                } else {
                    Ok(n as u32)
                }
            })
            .and_then(|d: Result<u32, ErrorKind>| match d {
                Ok(d) => Ok(d),
                Err(e) => Err(warp::reject::custom(e)),
            });
        let n = warp::query().map(|q: Query| q.n);

        let root = warp::get2().and(from).and(to).and(n);

        root.map(|start, duration, count| {
            json!({
                "start": format_time(start),
                "end": format_time(duration),
                "codes": sample_timecodes(start, duration, count)
            })
            .to_string()
        })
        .boxed()
    }
}

#[derive(Deserialize)]
struct Query {
    n: u32,
}
