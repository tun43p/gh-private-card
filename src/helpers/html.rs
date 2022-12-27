use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

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

/// **Create HTML card from GitHub repository**
pub fn create_github_repository_card(repository: &Repository) -> String {
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

    create_html_file(&context, "github_repository.html".to_string())
        .expect("error creating html file");

    // TODO(tun43p): Return an image
    match TEMPLATES.render("github_repository.html", &context) {
        Ok(html) => html,
        Err(error) => {
            eprintln!("{}", error.to_string());
            ::std::process::exit(1);
        }
    }
}

fn create_html_file(context: &Context, template: String) -> std::io::Result<()> {
    if Path::new("build/").exists() {
        remove_dir_all("build").expect("error deleting build directory");
    }

    create_dir_all("build/").expect("error creating build directory");
    let mut file = File::create("build/render.html").expect("error creating file");

    match TEMPLATES.render(&template, &context) {
        Ok(html) => file.write_all(html.as_bytes()),
        Err(error) => {
            eprintln!("{}", error.to_string());
            ::std::process::exit(1);
        }
    }
}
