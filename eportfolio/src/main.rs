use std::collections::HashMap;

pub mod render_markdown;

use render_markdown::render_md;
use unchained::{
    error::Error,
    router::{HTTPVerb::*, Request, Response, ResponseContent, Route},
    server::Server,
    templates::{
        context::ContextTree,
        render::{load_template, RenderOptions},
    },
};

use std::time::{SystemTime, UNIX_EPOCH};

const SEC_PER_DAY: u64 = 60 * 60 * 24;
const SEC_PER_YEAR: u64 = SEC_PER_DAY * 365;

fn current_year() -> u64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
    let seconds_since_epoch = since_epoch.as_secs();

    1970 + (seconds_since_epoch / SEC_PER_YEAR)
}

const BIRTH_YEAR: u64 = 2002 - 1970; // 1970 is the epoch year
/// Adding `BIRTH_DAY` to `BIRTH_MONTH` full months
const BIRTH_MONTH: u64 = 6;
const BIRTH_DAY: u64 = 19;
const AVERAGE_MONTH_DAYS: f64 = 30.436875; // Average days in a month

fn current_age() -> u64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();

    let since_birthday = (since_epoch.as_secs() as f64)
        - ((BIRTH_YEAR * 365 * SEC_PER_DAY) as f64
            + (BIRTH_MONTH as f64) * AVERAGE_MONTH_DAYS * (SEC_PER_DAY as f64)
            + (BIRTH_DAY * SEC_PER_DAY) as f64);

    (since_birthday / SEC_PER_YEAR as f64) as u64
}

fn handle_error(e: &Error) -> String {
    match e {
        Error::InvalidParams(s) => s.to_string(),
        Error::LoadFile(s) => s.to_string(),
        Error::ParseTemplate(s) => s.to_string(),
        Error::Connection(s) => s.to_string(),
    }
}

fn load_tmpl_and_handle_error(path: &str, context: Option<HashMap<String, ContextTree>>) -> String {
    match load_template(path, context, &RenderOptions::empty()) {
        Ok(template) => template.to_string(),
        Err(e) => handle_error(&e),
    }
}

fn create_skill(
    id: &str,
    name: &str,
    description: &str,
    score: isize,
    image_path: &str,
) -> ContextTree {
    [
        ("id", ContextTree::from(id)),
        ("name", name.into()),
        ("description", description.into()),
        ("score", score.into()),
        ("image", image_path.into()),
        ("percentage", (score * 100 / 5).into()),
    ]
    .into()
}

fn create_experience(
    id: &str,
    title: &str,
    description: &str,
    image_path: &str,
    date_start: &str,
    date_end: &str,
    demo_link: &str,
    source_link: &str,
    tech: Vec<&str>,
) -> ContextTree {
    [
        ("id", ContextTree::from(id)),
        ("title", title.into()),
        ("description", description.into()),
        ("image", image_path.into()),
        ("date_start", date_start.into()),
        ("date_end", date_end.into()),
        ("demo_link", demo_link.into()),
        ("source_link", source_link.into()),
        ("tech", tech.into()),
    ]
    .into()
}

fn create_course(course_id: &str, title: &str, image_path: &str) -> ContextTree {
    [
        ("course_id", course_id),
        ("title", title),
        ("image", image_path),
    ]
    .into()
}

fn folder_access(path: &str) -> Route {
    Route::new(GET, path, ResponseContent::FolderAccess)
}

fn main() {
    let mut context_base = HashMap::new();

    let current_year: isize = current_year().try_into().unwrap();
    context_base.insert("current_year".to_string(), current_year.into());
    context_base.insert(
        "brainmade".to_string(),
        [
            ("link", "https://brainmade.org"),
            (
                "tooltip",
                "Yes, I did watch the youtube video from No Boilerplate...",
            ),
        ]
        .into(),
    );

    let page_links = [
        [("href", "/#about"), ("label", "About me")],
        [("href", "/experience"), ("label", "Experience")],
        [("href", "/skills"), ("label", "Skills")],
        [("href", "/courses"), ("label", "Courses")],
    ];
    context_base.insert("page_links".to_string(), page_links.into());

    let mut context_landing = context_base.clone();
    let mut context_skills = context_base.clone();
    let mut context_experience = context_base.clone();
    let mut context_courses = context_base.clone();

    context_landing.insert(
        "carl_images".to_string(),
        [
            [
                ("path", "cafe-midi.webp"),
                ("alt", "Me and goat at Cafe du Midi"),
            ],
            [("path", "abtswoudse-bos.webp"), ("alt", "Park in Delft")],
            [("path", "zeeland-beach.webp"), ("alt", "Beach in Zeeland")],
        ]
        .into(),
    );
    context_landing.insert("age".into(), (current_age() as isize).into());

    context_skills.insert("skills".to_string(), [
        create_skill("django", "Django", "I have used Django in various projects in Index, Hackerspace, and Ei Solutions. It has been my Go To framework for backend developement because of its simplicity, scalability, and effeciency.", 5, "django.webp"),
        create_skill("java", "Java", "Java was used extensively at NTNU and was often required for school projects with Spring Boot, Maven and more.", 5, "java.webp"),
        create_skill("docker", "Docker", "I have used docker in several projects with Index, Hackerspace, and Ei Solutions. It has been very useful in both development and deployment. Yet, it is incredibly complex to master. My skill with Docker centers around using Compose and creating Dockerfiles.", 4, "docker.webp"),
        create_skill("python", "Python", "Python was my my first introduction to programming with a clear goal in mind. It has been used in my projects with Django. It was also used in the CS50-AI course with Tensorflow.", 4, "python.webp"),
        create_skill("typescript", "Type Script", "TypeScript has been used in all Front end projects. In high school, I was introduced to JavaScript, but after learning TypeScript, I have understood that I can never go back", 4, "typescript.webp"),
        create_skill("next", "Next", "This SSR framework was used to build my bachelor thesis product in addition to the landing page for Ei Solutions.", 3, "next.webp"),
        create_skill("postgis", "PostGIS", "This Postgres extension has been used to store and query spatial data in Ei Solutions. Postgres with PostGIS is by far the best relational geospatial database.", 3, "postgis.webp"),
        create_skill("qgis", "QGIS", "In Ei Solutions, I used QGIS to pre-process datasets before storing them in a PostGIS database.", 3, "qgis.webp"),
        create_skill("rust", "Rust", "I enjoy writing in this language and have created some fun projects with it, including this website.", 2, "rust.webp"),
    ].into());

    context_experience.insert("experience_list".to_string(), [
        create_experience("unchained", "Unchained router and templater", "Wanted to remove as much JavaScript from the website as possible so created a router and html template library that this website is created with.", "unchained.webp", "Mar 2024", "", "https://gutzkow.com", "https://github.com/cjgutz/unchained", vec!["Rust", "Docker"]),
        create_experience("hackerspace-deputy", "Deputy Commander - Hackerspace NTNU", "The deputy commander, together with the lead and the financial manager, had the responsibility to administer the organization. We made equipment available for students, organized events like the general assembly, and created an environment for students to learn. The last few months, I took the lead role as the previous leader stepped down.", "hackerspace.webp", "Mar 2023", "Mar 2024", "https://hackerspace-ntnu.no", "https://github.com/hackerspace-ntnu", vec![]),
        create_experience("telescope", "Telescope", "We started as two developers and a project manager who created the first prototype for an application. The application helps property developers manage a risk and vulnerability assessment of physical climate risk and biodiversity. I had responsibility for the Back-end and managed analysis using large amounts of geodata in a postGIS database. In the summer of 2023, with more teamates, we rewrote the entire application with a higher priority on user experience. This tought me a great deal about creating applications that scale and easily adapts to changing circumstances and customers.", "telescope.webp", "Jun 22", "", "https://telescope.eco", "", vec!["Django", "PostGIS", "QGIS", "Docker"]),
        create_experience("hackerspace-devops", "DevOps Member and Team Leader - Hackerspace NTNU", "For a year I managed the DevOps team at Hackerspace NTNU. I got into the role after one semester. I had responsibility for the development lifecycle, server infrastructure and the team's well-being. When I became deputy leader of the organization, I continued working with DevOps.", "hackerspace.webp", "Aug 2021", "Mar 2024", "https://hackerspace-ntnu.no", "https://github.com/hackerspace-ntnu", vec!["Django", "Docker"]),
        create_experience("tihlde-index", "Programmer with TIHLDE Index", "Worked as a Back-end developer for index.", "tihlde.webp", "Aug 2021", "Jun 2022", "https://tihlde.org", "https://github.com/tihlde/lepton", vec!["Django", "Docker"]),
    ].into());

    context_courses.insert(
        "course_pages".to_string(),
        [
            create_course("CS4515", "3D Computer Graphics and Animation", ""),
            create_course("CS4505", "Software Architecture", ""),
            create_course("DSAIT4005", "Machine and Deep Learning", ""),
            create_course("CS4510", "Formal Reasoning about Software", ""),
        ]
        .into(),
    );

    let start = std::time::Instant::now();
    let landing =
        load_tmpl_and_handle_error("templates/landing.html", Some(context_landing.clone()));
    let skills = load_tmpl_and_handle_error("templates/skills.html", Some(context_skills));
    let experience =
        load_tmpl_and_handle_error("templates/experience.html", Some(context_experience));
    let courses = load_tmpl_and_handle_error("templates/course-list.html", Some(context_courses));
    let page_404 = load_tmpl_and_handle_error("templates/404.html", Some(context_landing));
    let duration = start.elapsed();
    println!("Finished rendering after {} s", duration.as_secs_f64());

    let routes = vec![
        Route::new(GET, "/", ResponseContent::Str(landing)),
        Route::new(GET, "/skills", ResponseContent::Str(skills)),
        Route::new(GET, "/experience", ResponseContent::Str(experience)),
        Route::new(GET, "/courses", ResponseContent::Str(courses)),
        Route::new(
            GET,
            "courses/:courseid",
            ResponseContent::FromRequest({
                let page_404 = page_404.clone();
                Box::new(move |req: Request| {
                    let md = if let Some(courseid) = req.path_params.get("courseid") {
                        let mut ctx = context_base.clone();
                        let path = format!("templates/markdown/courses/{}.md", courseid);
                        ctx.insert("course_md_path".to_string(), path.into());
                        render_md("templates/course-detail.html", Some(ctx)).ok()
                    } else {
                        None
                    };
                    let is_some = md.is_some();
                    Response::new(
                        Some(md.unwrap_or(page_404.clone())),
                        if is_some { 200 } else { 404 },
                    )
                })
            }),
        ),
        folder_access("/images/*"),
        folder_access("/Poppins/Poppins-Regular.ttf"),
        folder_access("favicon.ico"),
        folder_access("cv.pdf"),
        folder_access("robots.txt"),
        folder_access("templates/css/*"),
        Route::new(
            GET,
            "/*",
            ResponseContent::FromRequest(Box::new(move |_req: Request| {
                Response::new(Some(page_404.clone()), 404)
            })),
        ),
    ];
    let mut server = Server::new(routes);
    server.add_default_header("Cache-Control", "max-age=300");
    server.add_default_header("Accept-Encoding", "gzip, compress, br");
    server.listen();
}
