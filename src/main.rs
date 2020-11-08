#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod docker;

use docker::DockerClient;

#[get("/")]
async fn index() -> String {
    let docker_client = DockerClient::default();

    let containers = docker_client.list().await.unwrap();
    let names = containers
        .iter()
        .map(|c| c.names[0].clone())
        .collect::<Vec<String>>()
        .join(", ");

    format!("Found active containers {}\n", names)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
