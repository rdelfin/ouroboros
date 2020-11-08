#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod docker;
mod response;

use crate::docker::DockerClient;
use crate::response::Container;
use rocket::response::content;
use serde_json;

#[get("/")]
async fn index() -> content::Json<String> {
    let docker_client = DockerClient::default();

    let containers = docker_client.list().await.unwrap();
    let names = containers
        .iter()
        .map(|c| Container {
            name: c.names[0].clone(),
            id: c.id.clone(),
        })
        .collect::<Vec<Container>>();

    content::Json(serde_json::to_string(&names).unwrap())
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
