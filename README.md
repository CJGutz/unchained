# Unchained
*Template renderer and router*

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

```html
<!-- templates/landing-page.html -->
{* component templates/base.html {
    <div class="grid grid-cols-2 gap-4 m-5">
        {* for button in page_links {
            {* component templates/front-button.html label=button.label link=button.href *}
        } *}
    </div>
    <div class="w-full mt-96 sm-p-10 p-4">
        <h1 id="about" class="text-4xl font-bold">About me</h1>
        <div class="p-10 flex flex-row gap-x-3 overflow-x-scroll snap-x snap-mandatory">
            {* for image in images {
                <img alt="{* image.alt *}"  src="/images/{* image.path *}" width="500" height="500">
             } *}
        </div>
    </div>
} *}
```
