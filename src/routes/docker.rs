use crate::structs::{
    Container, ContainerCreateRequest, ContainerCreateResponse, ImageCreateRequest,
    ImageCreateResponse,
};
use bollard::{
    container::{Config as ContainerConfig, CreateContainerOptions},
    errors::Error as BollardError,
    image::CreateImageOptions,
    Docker,
};
use rocket::futures::StreamExt;
use rocket_contrib::json::Json;
use std::collections::HashMap;

#[get("/list", format = "json")]
pub async fn list() -> Json<Vec<Container>> {
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

    Json(containers)
}

#[post("/image/create", format = "json", data = "<req>")]
pub async fn create_image(req: Json<ImageCreateRequest>) -> Json<ImageCreateResponse> {
    let docker_client = get_client().unwrap();

    let img_results = docker_client.create_image(
        Some(CreateImageOptions {
            from_image: req.name.clone(),
            ..Default::default()
        }),
        None,
        None,
    );
    img_results
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, BollardError>>()
        .unwrap();

    Json(ImageCreateResponse { ok: true })
}

#[post("/container/create", format = "json", data = "<req>")]
pub async fn create_container(req: Json<ContainerCreateRequest>) -> Json<ContainerCreateResponse> {
    let docker_client = get_client().unwrap();

    let res = docker_client
        .create_container(
            Some(CreateContainerOptions {
                name: req.name.clone(),
            }),
            ContainerConfig {
                exposed_ports: Some(
                    req.port_mappings
                        .iter()
                        .map(|mapping| {
                            (
                                format!("{}/{}", mapping.container_port, mapping.protocol),
                                HashMap::new(),
                            )
                        })
                        .collect(),
                ),
                env: Some(
                    req.environment
                        .iter()
                        .map(|(k, v)| match v {
                            Some(v) => format!("{}={}", k, v),
                            None => k.to_string(),
                        })
                        .collect(),
                ),
                volumes: Some(
                    req.volumes
                        .iter()
                        .map(|v| (v.container_dir.clone(), HashMap::new()))
                        .collect(),
                ),
                image: Some(req.image.clone()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    Json(ContainerCreateResponse {
        id: res.id,
        warnings: res.warnings,
    })
}

fn get_client() -> Result<Docker, BollardError> {
    Ok(Docker::connect_with_local_defaults()?)
}
