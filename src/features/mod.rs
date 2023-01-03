pub(crate) mod github;
pub(crate) mod healthcheck;

use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(template) => template,
            Err(error) => {
                eprintln!("{}", error);
                ::std::process::exit(1);
            }
        }
    };
}
