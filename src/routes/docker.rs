use crate::{docker::DockerClient, response::Container};
use anyhow::Result;
use rocket::{
    http::{ContentType, Header},
    response::content,
};

#[get("/list")]
pub async fn list() -> content::Json<String> {
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
