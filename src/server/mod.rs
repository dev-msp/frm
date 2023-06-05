use lru::LruCache;
use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};
use serde_json::json;
use thiserror::Error;
use warp::{self, filters::BoxedFilter, path, Filter, Rejection, Reply};

use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::ffmpeg::ErrorKind;
use crate::ffmpeg::{
    cmd::Command,
    duration,
    frame::{self, Frame},
    sequence::Sequence,
};

#[allow(dead_code)]
pub struct SampleWindow {
    pub n: u32,
    pub start: u32,
    pub end: Option<u32>,
}

fn request_frame(
    handle: Arc<Mutex<ServerHandle>>,
    file_str: String,
    i: u32,
) -> Result<Frame, ErrorKind> {
    {
        let Ok(mut handle) = handle.lock() else {
            todo!()
        };

        if let Some(frame) = handle.cache.get(&i) {
            return Ok(frame.clone());
        }
    };

    let other_handle = handle.clone();
    other_handle
        .lock()
        .unwrap()
        .pool
        .install(move || -> Result<Frame, ErrorKind> {
            {
                let Ok(mut handle) = handle.lock() else {
            todo!()
        };

                if let Some(frame) = handle.cache.get(&i) {
                    return Ok(frame.clone());
                }
            };

            let mut frame = frame::Frame::new(file_str.as_str(), i)?;
            frame.read()?;

            let Ok(mut handle) = handle.lock() else {
                todo!();
            };

            handle.cache.push(i, frame.clone());
            Ok(frame)
        })
}
fn get_subs(file_str: String, s: u32, e: u32) -> String {
    match Sequence::subtitles(file_str, s, e).execute() {
        Ok(bs) => match std::str::from_utf8(&bs.stdout) {
            Ok(s) => s.to_string(),
            Err(e) => json!({ "error": e.to_string() }).to_string(),
        },
        Err(e) => json!({ "error": e.to_string() }).to_string(),
    }
}

pub struct FrameServer {
    file: String,
    handle: Arc<Mutex<ServerHandle>>,
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

struct ServerHandle {
    pool: ThreadPool,
    cache: LruCache<u32, Frame>,
}

impl FrameServer {
    pub fn new(file: String, num_threads: usize) -> Result<FrameServer, Error> {
        let pool = ThreadPoolBuilder::default()
            .num_threads(num_threads)
            .build()?;
        let cache = LruCache::new(200.try_into().unwrap());
        Ok(FrameServer {
            file,
            handle: Arc::new(Mutex::new(ServerHandle { pool, cache })),
        })
    }

    pub fn serve(self) {
        let root = self
            .context_route()
            .or(self.image_route())
            .or(self.srt_route());
        warp::serve(root).run(([127, 0, 0, 1], 3030));
    }

    fn image_route(&self) -> BoxedFilter<(impl Reply,)> {
        let file = self.file.clone();
        let handle = self.handle.clone();
        path("frame")
            .and(warp::get())
            .and(path::param())
            .and(path::end())
            .map(move |i| request_frame(handle, file, i).map_err(warp::reject::custom))
            .and_then(|frame: Result<Frame, Rejection>| async move {
                <Result<Vec<u8>, Rejection>>::Ok(frame?.write()?)
            })
            .boxed()
    }

    fn srt_route(&self) -> BoxedFilter<(impl Reply,)> {
        let file = self.file.clone();
        let pool = self.pool.clone();

        let get_subs = move |s, e| pool.install(|| get_subs(file.clone(), s, e));

        path("subtitles")
            .and(warp::get())
            .and(path::param())
            .and(path::param())
            .map(get_subs)
            .boxed()
    }

    fn context_route(&self) -> BoxedFilter<(impl Reply,)> {
        let file = self.file.clone();
        path("context")
            .and(path::end())
            .map(move || duration(&file))
            .and_then(|duration: Result<Duration, ErrorKind>| async move {
                match duration {
                    Ok(d) => Ok(json!({ "duration": d.as_millis() as u32 }).to_string()),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            })
            .boxed()
    }
}
