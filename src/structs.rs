use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub id: String,
    pub command: String,
}
