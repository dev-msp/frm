use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use hyper::header::CONTENT_TYPE;
use lru::LruCache;
use serde_json::json;
use thiserror::Error;
use tokio::sync::Mutex;

use std::convert::TryInto;
use std::sync::Arc;

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
}

#[derive(Debug, Error)]
pub enum Error {
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

#[derive(Clone)]
struct AppState {
    file: String,
    cache: Arc<Mutex<LruCache<u32, Frame>>>,
}

impl AppState {
    async fn request_frame(&self, i: u32) -> Result<Vec<u8>, ErrorKind> {
        {
            let mut cache = self.cache.lock().await;
            if let Some(frame) = cache.get_mut(&i) {
                // println!("Found {i} in cache");
                return frame.write();
            }
        };

        let mut frame = frame::Frame::new(self.file.as_str(), i)?;

        let mut fc = frame.clone();
        tokio::task::spawn_blocking(move || fc.read())
            .await
            .map_err(|_| ErrorKind::Unhandled("oops".into()))??;

        let mut cache = self.cache.lock().await;

        frame.read()?;

        cache.push(i, frame.clone());
        // println!("Pushed {i} to cache");
        frame.write()
    }
}

async fn handle_image(State(state): State<AppState>, Path(timestamp): Path<u32>) -> Response {
    match state.request_frame(timestamp).await {
        Ok(bytes) => ([(CONTENT_TYPE, "image/png")], bytes).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

impl FrameServer {
    pub fn new(file: String) -> Result<FrameServer, Error> {
        Ok(FrameServer { file })
    }

    pub async fn serve(self) {
        let app: Router<AppState, axum::body::Body> = Router::new()
            .route("/frame/:timestamp", get(handle_image))
            .route(
                "/context",
                get(|State(state): State<AppState>| async move {
                    let d = duration(&state.file);
                    match d {
                        Ok(d) => axum::Json(json!({ "duration": d.as_millis() as u32 })),
                        Err(e) => axum::Json(json!({ "error": e.to_string() })),
                    }
                }),
            );

        let builder = axum::Server::bind(&"127.0.0.1:3030".parse().unwrap());
        let ready_app = app.with_state::<()>(AppState {
            file: self.file,
            cache: Arc::new(Mutex::new(LruCache::new(1200.try_into().unwrap()))),
        });
        let server = builder.serve(ready_app.into_make_service());
        server.await.unwrap();
    }

    // fn srt_route(&self) -> BoxedFilter<(impl Reply,)> {
    //     let file = self.file.clone();
    //     let pool = self.pool.clone();
    //
    //     let get_subs = move |s, e| pool.install(|| get_subs(file.clone(), s, e));
    //
    //     path("subtitles")
    //         .and(warp::get())
    //         .and(path::param())
    //         .and(path::param())
    //         .map(get_subs)
    //         .boxed()
    // }
}
