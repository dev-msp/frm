mod state;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use hyper::header::CONTENT_TYPE;
use serde_json::json;
use thiserror::Error;

use crate::ffmpeg::{cmd::Command, duration, sequence::Sequence};

use self::state::AppState;

#[allow(dead_code)]
pub struct SampleWindow {
    pub n: u32,
    pub start: u32,
    pub end: Option<u32>,
}

fn get_subs(file_str: String, s: usize, e: usize) -> String {
    match Sequence::subtitles(file_str, s, e).execute() {
        Ok(bs) => match std::str::from_utf8(&bs.stdout) {
            Ok(s) => s.to_string(),
            Err(e) => json!({ "error": e.to_string() }).to_string(),
        },
        Err(e) => json!({ "error": e.to_string() }).to_string(),
    }
}

#[derive(Debug)]
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

#[tracing::instrument(skip_all)]
async fn handle_image(State(state): State<AppState>, Path(timestamp): Path<usize>) -> Response {
    match state.request_frame(timestamp).await {
        Ok(bytes) => ([(CONTENT_TYPE, "image/png")], bytes).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[tracing::instrument(skip(state))]
async fn handle_image_range(
    State(state): State<AppState>,
    Path((from, to, n)): Path<(usize, usize, usize)>,
) -> Response {
    let step = (to - from) / n;
    match state.ingest_frame_range(from, n, step).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

impl FrameServer {
    pub fn new(file: String) -> Result<FrameServer, Error> {
        Ok(FrameServer { file })
    }

    #[tracing::instrument(skip_all)]
    pub async fn serve(self) {
        let app: Router<AppState, axum::body::Body> = Router::new()
            .route("/frame/:at", get(handle_image))
            .route("/frames/:from/:to/:step", get(handle_image_range))
            .route(
                "/context",
                get(|State(state): State<AppState>| async move {
                    let d = duration(state.file());
                    match d {
                        Ok(d) => axum::Json(json!({ "duration": d.as_millis() as u32 })),
                        Err(e) => axum::Json(json!({ "error": e.to_string() })),
                    }
                }),
            );

        let builder = axum::Server::bind(&"127.0.0.1:3030".parse().unwrap());
        let ready_app = app.with_state::<()>(AppState::new(self.file.clone(), 1200));
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
