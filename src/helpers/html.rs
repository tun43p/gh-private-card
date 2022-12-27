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

    // TODO(tun43p): Convert as image
    match TEMPLATES.render("github_repository.html", &context) {
        Ok(template) => template,
        Err(error) => {
            eprintln!("{}", error.to_string());
            error.to_string()
        }
    }
}
