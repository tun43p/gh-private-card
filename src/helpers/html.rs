use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir_all, remove_dir_all, File},
    hash::{Hash, Hasher},
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use lazy_static::lazy_static;
use tera::{Context, Tera};

use crate::models::repository::Repository;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*") {
            Ok(template) => template,
            Err(error) => {
                eprintln!("{}", error.to_string());
                ::std::process::exit(1);
            }
        }
    };
}

pub struct HtmlData {
    pub content: String,
    pub file_path: String,
    pub hash: u64,
}

pub struct PngData {
    pub file_path: String,
    pub hash: u64,
}

/// **Create HTML card from GitHub repository**
///
/// Retuns a [(HtmlData, PngData)]
pub fn create_github_repository_card(repository: &Repository) -> (HtmlData, PngData) {
    if Path::new("build/").exists() {
        remove_dir_all("build").expect("error deleting build directory");
    }

    create_dir_all("build").expect("error creating directory");

    let mut context = Context::new();

    let full_name = Vec::from_iter(repository.full_name.split("/").map(String::from));
    let owner = full_name.first().expect("error getting repository owner");
    let name = full_name.last().expect("error getting repository name");

    context.insert("owner", owner);
    context.insert("name", name);
    context.insert("description", &repository.description);
    context.insert("avatar", &repository.owner.avatar_url);
    context.insert("issues", &repository.open_issues_count);
    context.insert("url", &repository.html_url);

    let html_data = create_html_file(&context, "github_repository.html".to_string())
        .expect("error creating html file");

    let png_data = create_png_file(&html_data).expect("error creating png file");

    (html_data, png_data)
}

/// **Render and create an HTML file builed on top of an HTML template**
///
/// Returns a [Result] of [HtmlData] for the `(hash, file_path)`
fn create_html_file(context: &Context, template: String) -> std::io::Result<HtmlData> {
    let hash = create_hash(format!(
        "{data}\n{timestamp}",
        data = &template,
        timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error getting timestamp")
            .as_millis()
    ));
    let file_path = format!("build/{}.html", hash.to_string());

    let html = TEMPLATES
        .render(&template, context)
        .expect("error rendering html template");

    let mut file = File::create(&file_path).expect("error creating html file");
    file.write_all(html.as_bytes())
        .expect("error writing on html file");

    Ok(HtmlData {
        content: html,
        file_path: format!(
            "file:///{pwd}/{file_path}",
            pwd = std::env::current_dir()
                .expect("error getting current directory")
                .display(),
            file_path = file_path,
        ),
        hash,
    })
}

/// **Convert HTML file to a PNG image**
///
/// Returns a [Result] of [String] for the `file_path`
fn create_png_file(html_data: &HtmlData) -> std::io::Result<PngData> {
    let file_path = format!("build/{}.png", html_data.hash.to_string());

    let browser = Browser::default().expect("error creating browser");

    let tab = browser
        .wait_for_initial_tab()
        .expect("error during the wait of the initial tab");

    tab.navigate_to(&html_data.file_path)
        .expect("error navigating to local file");

    let element = tab
        .wait_for_element("body")
        .expect("error getting body element");

    let image = element
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)
        .expect("error capturing screenshot");

    let mut file = File::create(&file_path).expect("error creating png file");
    file.write_all(&image).expect("error creating png file");

    Ok(PngData {
        hash: html_data.hash,
        file_path: format!(
            "file:///{pwd}/{file_path}",
            pwd = std::env::current_dir()
                .expect("error getting current directory")
                .display(),
            file_path = file_path,
        ),
    })
}

/// **Create an hash from an object to **
///
/// Returns a [u64] for the `hash`
fn create_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
