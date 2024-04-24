use std::collections::HashMap;

use unchained::{
    error::Error,
    router::{HTTPVerb::*, ResponseContent, Route, Server},
    templates::{
        context::{ctx_map, ctx_str, ctx_vec},
        render::template,
    },
};

fn main() {
    let mut context = HashMap::new();

    context.insert(
        "page_links".to_string(),
        ctx_vec(vec![
            ctx_map([("href", ctx_str("/#about")), ("label", ctx_str("About me"))]),
            ctx_map([
                ("href", ctx_str("/experience")),
                ("label", ctx_str("Experience")),
            ]),
            ctx_map([("href", ctx_str("/skills")), ("label", ctx_str("Skills"))]),
        ]),
    );

    context.insert(
        "carl_images".to_string(),
        ctx_vec(vec![
            ctx_map([
                ("path", ctx_str("ski.webp")),
                ("alt", ctx_str("Me on a randonee ski trip")),
            ]),
            ctx_map([
                ("path", ctx_str("storheia.webp")),
                ("alt", ctx_str("Me on top of Storheia")),
            ]),
            ctx_map([
                ("path", ctx_str("broa.webp")),
                ("alt", ctx_str("Gamlebroa")),
            ]),
            ctx_map([
                ("path", ctx_str("hovedbygget.webp")),
                ("alt", ctx_str("Me in front of Hovedbygget NTNU")),
            ]),
            ctx_map([
                ("path", ctx_str("gotland-gaard.webp")),
                ("alt", ctx_str("Me on Gotland")),
            ]),
            ctx_map([
                ("path", ctx_str("index-intervju.webp")),
                ("alt", ctx_str("Me at interview with Tihlde Index")),
            ]),
            ctx_map([
                ("path", ctx_str("piz-boe.webp")),
                ("alt", ctx_str("Me on top of Piz Boe")),
            ]),
        ]),
    );

    let start = std::time::Instant::now();
    let template = template("templates/landing.html", Some(context));
    let duration = start.elapsed();
    println!("Finished rendering after {} s", duration.as_secs_f64());

    let routes = vec![
        Route::new(
            GET,
            "/",
            ResponseContent::Str(match &template {
                Ok(template) => template.to_string(),
                Err(e) => match e {
                    Error::InvalidParams(s) => s.to_string(),
                    Error::LoadFile(s) => s.to_string(),
                    Error::ParseTemplate(s) => s.to_string(),
                },
            }),
        ),
        Route::new(GET, "/images/*", ResponseContent::FolderAccess),
    ];
    let server = Server::new(routes);
    server.listen();
}
