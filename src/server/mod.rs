use handlebars::Handlebars;
use warp::{self, Filter};

use serde::Serialize;

use std::error::Error;
use std::sync::Arc;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    hbs.render(template.name, &template.value)
        .map(|s| Some(s))
        .unwrap_or_else(|err| Some(err.description().to_owned()))
        .map(|s| warp::reply::html(s))
        .unwrap()
}

pub fn serve() {
    let template = "<!DOCTYPE html>
                    <html>
                      <head>
                        <title>Warp Handlebars template example</title>
                      </head>
                      <body>
                        <h1>Hello {{user}}!</h1>
                      </body>
                    </html>";

    let mut hb = Handlebars::new();
    hb.register_template_string("template.html", template)
        .unwrap();

    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    let route = warp::get2()
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({"user": "Warp"}),
        })
        .map(handlebars);

    warp::serve(route).run(([127, 0, 0, 1], 3030));
}
