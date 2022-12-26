use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}
