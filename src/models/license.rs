use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: String,
    node_id: String,
}
