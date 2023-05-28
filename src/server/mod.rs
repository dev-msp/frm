use handlebars::Handlebars;
use serde::Serialize;
use warp::{self, filters::BoxedFilter, Filter, Reply};

use std::collections::HashMap;
use std::sync::Arc;

use super::ffmpeg::{duration, frame};

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl Reply
where
    T: Serialize,
{
    hbs.render(template.name, &template.value)
        .map(Some)
        .unwrap_or_else(|err| Some(err.to_string()))
        .map(warp::reply::html)
        .unwrap()
}

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
    let n_f = n as f32;
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
type FrameHash<'a> = HashMap<u32, frame::Frame<'a>>;

fn render_frame(file_str: String, i: u32) -> impl Reply {
    frame::Frame::new(file_str.as_ref(), i)
        .and_then(|frm| frm.write())
        .map(|r| warp::reply::with_header(r, "Content-Type", "image/jpeg"))
        .unwrap()
}

pub struct FrameServer {
    file: String,
}

impl<'a> FrameServer<'a> {
    pub fn new(file: &str) -> FrameServer {
        FrameServer { file }
    }

    pub fn serve(&self) {
        let root = self.page_route().or(self.image_route());
        warp::serve(root).run(([127, 0, 0, 1], 3030));
    }

    fn image_route(&self) -> BoxedFilter<(impl Reply,)> {
        let file_arc = Arc::new(self.file.to_owned());
        let render_frame = move |i| render_frame(file_arc.clone(), i);

        warp::path("frame")
            .and(warp::get2())
            .and(warp::path::param())
            .map(render_frame)
            .boxed()
    }

    fn page_route(&self) -> BoxedFilter<(impl Reply,)> {
        let mut hb = Handlebars::new();
        hb.register_template_file("sample", "./templates/sample.hbs")
            .unwrap();

        let hb = Arc::new(hb);
        let handlebars = move |with_template| render(with_template, hb.clone());

        let file = self.file.clone();

        let root = warp::get2()
            .and(warp::path::end())
            .map(move || duration(&file))
            .and_then(|duration| match duration {
                Ok(d) => Ok((0, d as u32, 10)),
                Err(e) => Err(warp::reject::custom(e)),
            });

        let page = warp::path!("from" / u32 / "to" / u32).map(|from, to| (from, to, 10));
        root.or(page)
            .unify()
            .map(|(start, duration, count)| WithTemplate {
                name: "sample",
                value: json!({ "codes": sample_timecodes(start, duration, count) }),
            })
            .map(handlebars)
            .boxed()
    }
}
