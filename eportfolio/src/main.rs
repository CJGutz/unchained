use std::collections::HashMap;

use unchained::{
    error::{Error, WebResult},
    router::{HTTPVerb::*, ResponseContent, Route, Server},
    templates::{
        context::{ctx_map, ctx_str, ctx_vec, ContextTree, Primitive}, render::{load_template, RenderOptions}
    },
};

use std::time::{SystemTime, UNIX_EPOCH};

fn current_year() -> u64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
    let seconds_since_epoch = since_epoch.as_secs();
    let seconds_in_year: u64 = 60 * 60 * 24 * 365;

    1970 + (seconds_since_epoch / seconds_in_year)
}

fn handle_error(e: &Error) -> String {
    match e {
        Error::InvalidParams(s) => s.to_string(),
        Error::LoadFile(s) => s.to_string(),
        Error::ParseTemplate(s) => s.to_string(),
    }
}

fn create_skill(id: &str, name: &str, description: &str, score: isize, image_path: &str) -> ContextTree {
    ctx_map([
            ("id", ctx_str(id)),
            ("name", ctx_str(name)),
            ("description", ctx_str(description)),
            ("score", ContextTree::Leaf(Primitive::Num(score))),
            ("image", ctx_str(image_path)),
            ("percentage", ContextTree::Leaf(Primitive::Num(score * 100 / 5))),
        ])
}

fn create_experience(id: &str, title: &str, description: &str, image_path: &str) -> ContextTree {
    ctx_map([
            ("id", ctx_str(id)),
            ("title", ctx_str(title)),
            ("description", ctx_str(description)),
            ("image", ctx_str(image_path)),
            ("date_start", ctx_str("2023")),
            ("date_end", ctx_str("2024")),
            ("demo_link", ctx_str("https://gutzkow.com")),
            ("source_link", ctx_str("https://github.com/cjgutz")),
            ("tech", ctx_vec(vec![ctx_str("django"), ctx_str("docker")])),
            ("show_date_end", ContextTree::Leaf(Primitive::Bool(true))),
        ])
}

fn main() {
    let mut context_landing = HashMap::new();
    let mut context_skills = HashMap::new();
    let mut context_experience = HashMap::new();

    let current_year: isize = current_year().try_into().unwrap();
    context_landing.insert("current_year".to_string(), ContextTree::Leaf(Primitive::Num(current_year)));
    context_skills.insert("current_year".to_string(), ContextTree::Leaf(Primitive::Num(current_year)));
    context_experience.insert("current_year".to_string(), ContextTree::Leaf(Primitive::Num(current_year)));

    let page_links = ctx_vec(vec![
            ctx_map([("href", ctx_str("/#about")), ("label", ctx_str("About me"))]),
            ctx_map([
                ("href", ctx_str("/experience")),
                ("label", ctx_str("Experience")),
            ]),
            ctx_map([("href", ctx_str("/skills")), ("label", ctx_str("Skills"))]),
        ]);
    context_landing.insert("page_links".to_string(), page_links.clone());
    context_skills.insert("page_links".to_string(), page_links.clone());
    context_experience.insert("page_links".to_string(), page_links);


    context_landing.insert(
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

    context_skills.insert("skills".to_string(), ctx_vec(vec![
        create_skill("django", "Django", "I have used Django in various projects in Index, Hackerspace, and Ei Solutions. It has been my Go To framework for backend developement because of its simplicity, scalability, and effeciency.", 5, "django.png"),
        create_skill("docker", "Docker", "I have used docker in several projects with Index, Hackerspace, and Ei Solutions. It has been very useful in both development and deployment. Yet, it is incredibly complex to master. My skill with Docker centers around using Compose and creating Dockerfiles.", 4, "docker.png"),
        create_skill("java", "Java", "Java was used extensively at NTNU and was often required for school projects with Spring Boot, Maven and more.", 5, "java.png"),
        create_skill("next", "Next", "This SSR framework was used to build my bachelor thesis product in addition to the landing page for Ei Solutions.", 3, "next.png"),
        create_skill("postgis", "PostGIS", "This Postgres extension has been used to store and query spatial data in Ei Solutions. Postgres with PostGIS is by far the best relational geospatial database.", 3, "postgis.png"),
        create_skill("qgis", "QGIS", "In Ei Solutions, I used QGIS to pre-process datasets before storing them in a PostGIS database.", 3, "qgis.png"),
        create_skill("python", "Python", "Python was my my first introduction to programming with a clear goal in mind. It has been used in my projects with Django. It was also used in the CS50-AI course with Tensorflow.", 4, "python.svg"),
        create_skill("typescript", "Type Script", "TypeScript has been used in all Front end projects. In high school, I was introduced to JavaScript, but after learning TypeScript, I have understood that I can never go back", 4, "typescript.svg"),
        create_skill("rust", "Rust", "I enjoy writing in this language and have created some fun projects with it, including this website.", 2, "rust.svg"),
    ]));

    context_experience.insert("experience_list".to_string(), ctx_vec(vec![
        create_experience("eisolutions", "Ei Solutions", "I love Ei Solutions", "eisolutions.jpg"),
        create_experience("hackerspace", "Hackerspace NTNU", "I love Hackerspace NTNU", "hackerspace.png")
    ]));


    let start = std::time::Instant::now();
    // let landing = load_template("templates/landing.html", Some(context_landing), &RenderOptions::empty());
    // let skills = load_template("templates/skills.html", Some(context_skills), &RenderOptions::empty());
    let landing: WebResult<String> = WebResult::Err(Error::InvalidParams("Failed to load template".to_string()));
    let skills: WebResult<String> = WebResult::Err(Error::InvalidParams("Failed to load template".to_string()));
    let experience = load_template("templates/experience.html", Some(context_experience), &RenderOptions::empty());
    let duration = start.elapsed();
    println!("Finished rendering after {} s", duration.as_secs_f64());

    let routes = vec![
        Route::new(
            GET,
            "/",
            ResponseContent::Str(match &landing {
                Ok(template) => template.to_string(),
                Err(e) => handle_error(e),
            }),
        ),
        Route::new(
            GET,
            "/skills",
            ResponseContent::Str(match &skills {
                Ok(template) => template.to_string(),
                Err(e) => handle_error(e),
            }),
        ),
        Route::new(
            GET,
            "/experience",
            ResponseContent::Str(match &experience {
                Ok(template) => template.to_string(),
                Err(e) => handle_error(e),
            }),
        ),
        Route::new(GET, "/images/*", ResponseContent::FolderAccess),
        Route::new(GET, "/Poppins/Poppins-Regular.ttf", ResponseContent::Bytes(std::fs::read("Poppins/Poppins-Regular.ttf").unwrap())),
        Route::new(GET, "/favicon.ico", ResponseContent::Bytes(std::fs::read("favicon.ico").unwrap())),
    ];
    let server = Server::new(routes);
    server.listen();
}
