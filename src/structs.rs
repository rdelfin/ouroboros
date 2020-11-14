use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub id: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCreateRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCreateResponse {
    pub ok: bool,
}
