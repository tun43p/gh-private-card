pub(crate) mod github;

use lazy_static::lazy_static;
use tera::Tera;

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
