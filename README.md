# Unchained
## template renderer and router

> Created during the evenings of easter holiday

### Examples

```rust

fn main() {
    let mut context = HashMap::new();

    context.insert("page_links".to_string(), ctx_vec(vec![
        ctx_map([("href", ctx_str("/#about")), ("label", ctx_str("About me"))]),
        ctx_map([("href", ctx_str("/experience")), ("label", ctx_str("Experience"))]),
        ctx_map([("href", ctx_str("/skills")), ("label", ctx_str("Skills"))]),
    ]));

    let template = template("templates/landing-page.html", Some(context));

    let routes = vec![
        Route::new(GET, String::from("/"), ResponseContent::Create(Box::new(move |_req| {
            return match &template {
                Ok(template) => Response::new_200(template.to_string()),
                Err(_e) => panic!("Could not render template"),
            };
        }))),
        Route::new(GET, String::from("/images/*"), ResponseContent::None)
    ];
    start_server(routes, ServerOptions {address: None});
}
```
