use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Permissions {
    admin: bool,
    maintain: bool,
    push: bool,
    triage: bool,
    pull: bool,
}
