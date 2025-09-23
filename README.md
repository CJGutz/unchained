# Unchained
*Template renderer and router*

> Created during the evenings of Easter holiday


- Easy to use
- Easy to extend
- No extra dependencies

### Examples

```rust

fn main() {
    let mut context: HashMap<String, ContextTree> = HashMap::new();

    context.insert("page_links".to_string(), vec![
            [("href", "/#about"), ("label", "About me")],
            [("href", "/experience"), ("label", "Experience")],
            [("href", "/skills"), ("label", "Skills")],
        ].into()
    );

    let template = load_template("landing-page.html", Some(context), &RenderOptions::empty()).unwrap();

    let routes = vec![
        Route::new(GET, "/", ResponseContent::Str(template)),
        Route::new(GET, "/images/*", ResponseContent::FolderAccess),
    ];

    let mut server = Server::new(routes);
    server.set_address("localhost:8080");
    server.listen();
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
        <div class="p-10 flex flex-row gap-3 overflow-x-scroll">
            {* for image in images {
                <img alt="{* image.alt *}"  src="/images/{* image.path *}" width="500" height="500">
             } *}
        </div>
    </div>
} *}
```
