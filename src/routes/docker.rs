use crate::structs::Container;
use anyhow::Result;
use bollard::{CreateImageOptions, Docker};
use rocket::response::content;
use rocket_contrib::json::Json;

#[get("/list")]
pub async fn list() -> content::Json<String> {
    let docker_client = get_client().unwrap();

    let containers = docker_client
        .list_containers::<String>(None)
        .await
        .unwrap()
        .iter()
        .map(|c| Container {
            name: c.names.clone().expect("Container without a name")[0].clone(),
            id: c.id.clone().expect("Container without an id").clone(),
            command: c
                .command
                .clone()
                .expect("Container without a command")
                .clone(),
        })
        .collect::<Vec<_>>();

    content::Json(serde_json::to_string(&containers).unwrap())
}

fn get_client() -> Result<Docker> {
    Docker::connect_with_local_defaults()
}
