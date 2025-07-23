use std::fs;
use std::path::Path;

fn main() -> Result<(), markdown::message::Message>{
    // Input/output file paths

    let name = "portal";

    let input_path = format!("examples/docs/{name}.md");
    let output_path = format!("examples/public/{name}.html");

    // Create output directory if needed
    let output_dir = Path::new(&output_path).parent().unwrap();
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Read markdown file
    let markdown = fs::read_to_string(input_path).expect("Failed to read input file");

    // Build HTML from components
    let full_html = format!(
        r###"<!DOCTYPE html>
<html lang="en">
<head>{}</head>
        <body class="bg-background font-geist-sans text-foreground antialiased">
          <div class="p-6 lg:p-10">
            <div class="space-y-0.5">
              <h2 class="text-2xl font-bold tracking-tight">Settings</h2>
              <p class="text-muted-foreground">
                Manage your account settings and set e-mail preferences.
              </p>
            </div>
            <div class="border-border my-6 border-t"></div>
            <div class="flex gap-x-12">
              {}
              <div class="w-5/6">
                {}
              </div>
            </div>
          </div>
        </body>
</html>"###,
        get_html_head(),
        get_navigation_sidebar(),
        markdown::to_frankenui_with_options(&markdown, &markdown::Options::gfm()).unwrap()
    );

    // Write to output
    fs::write(output_path.clone(), full_html).expect("Failed to write output file");

    println!("✅ Generated: {output_path}");

    Ok(())
}

fn get_html_head() -> &'static str {
    r###"
  <meta charset="UTF-8">
  <title>FrankMark</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/franken-ui@2.1.0-next.16/dist/css/core.min.css"
    />
    <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/franken-ui@2.1.0-next.16/dist/css/utilities.min.css"
    />
    <script
    src="https://cdn.jsdelivr.net/npm/franken-ui@2.1.0-next.16/dist/js/core.iife.js"
    type="module"
    ></script>
    <script
    src="https://cdn.jsdelivr.net/npm/franken-ui@2.1.0-next.16/dist/js/icon.iife.js"
    type="module"
    ></script>

    <!-- Highlight.js -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/github.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>

    <script>
    const htmlElement = document.documentElement;

    const __FRANKEN__ = JSON.parse(localStorage.getItem("__FRANKEN__") || "{}");

    if (
        __FRANKEN__.mode === "dark" ||
        (!__FRANKEN__.mode &&
        window.matchMedia("(prefers-color-scheme: dark)").matches)
    ) {
        htmlElement.classList.add("dark");
    } else {
        htmlElement.classList.remove("dark");
    }

    htmlElement.classList.add(__FRANKEN__.theme || "uk-theme-neutral");
    htmlElement.classList.add(__FRANKEN__.radii || "uk-radii-md");
    htmlElement.classList.add(__FRANKEN__.shadows || "uk-shadows-sm");
    htmlElement.classList.add(__FRANKEN__.font || "uk-font-sm");
    htmlElement.classList.add(__FRANKEN__.chart || "uk-chart-default");
    </script>
"###
}


fn get_navigation_sidebar() -> &'static str {
    r###"
              <aside class="w-1/6">
                <ul
                  class="uk-nav uk-nav-secondary"
                  uk-switcher="connect: #component-nav; animation: uk-anmt-slide-left-sm"
                >
                  <li class="uk-active"><a href="#">Profile</a></li>
                  <li><a href="#">Account</a></li>
                  <li><a href="#">Appearance</a></li>
                  <li><a href="#">Notifications</a></li>
                  <li><a href="#">Display</a></li>
                </ul>
              </aside>
"###
}