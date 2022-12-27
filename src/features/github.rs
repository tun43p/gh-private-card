use super::TEMPLATES;
use crate::{helpers::hash::create_hash, models::github::Repository};
use headless_chrome::{protocol::cdp::Page, Browser};
use std::{fs::File, io::Write};
use tera::Context;

/// **Create HTML card from GitHub repository**
pub fn create_github_repository_card(repository: &Repository) -> String {
    let mut context = Context::new();

    // Parse full_name to get owner and name fields
    let full_name = Vec::from_iter(repository.full_name.split("/").map(String::from));
    let owner = full_name.first().expect("error getting repository owner");
    let name = full_name.last().expect("error getting repository name");

    // Update HTML context with our variables
    context.insert("owner", owner);
    context.insert("name", name);
    context.insert("description", &repository.description);
    context.insert("avatar", &repository.owner.avatar_url);
    context.insert("issues", &repository.open_issues_count);
    context.insert("url", &repository.html_url);

    // Render the template with the passed data
    let html_content = TEMPLATES
        .render("github_repository.html", &context)
        .expect("error rendering html template");

    // Create hash based on the HTML code
    let hash = create_hash(&html_content);

    // Get HTML and PNG file paths
    let html_file_path = format!("build/{}.html", hash.to_string());
    let png_file_path = format!("build/{}.png", hash.to_string());

    // TODO(tun43p): if png file path exists, returns current image

    // Save HTML file
    let mut html_file = File::create(&html_file_path).expect("error creating html file");
    html_file
        .write_all(html_content.as_bytes())
        .expect("error writing on html file");

    // Create web browser
    let browser = Browser::default().expect("error creating browser");

    // Go to the html file path and take a screenshot of the body element
    let png_content = browser
        .wait_for_initial_tab()
        .expect("error during the wait of the initial tab")
        .navigate_to(&format!(
            "file:///{0}/{1}",
            std::env::current_dir()
                .expect("error getting current directory")
                .display(),
            &html_file_path
        ))
        .expect("error navigating to local file")
        .wait_for_element("body")
        .expect("error getting body element")
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)
        .expect("error capturing screenshot");

    // Save PNG file
    let mut file = File::create(&png_file_path).expect("error creating png file");
    file.write_all(&png_content)
        .expect("error creating png file");

    // TODO(tun43p): Returns new image
    html_content
}
