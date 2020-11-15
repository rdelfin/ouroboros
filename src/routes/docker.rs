use crate::structs::{
    Container, ContainerCreateRequest, ContainerCreateResponse, ContainerRemoveRequest,
    ContainerRemoveResponse, ContainerStartRequest, ContainerStartResponse, ContainerStopRequest,
    ContainerStopResponse, ImageCreateRequest, ImageCreateResponse,
};
use bollard::{
    container::{
        Config as ContainerConfig, CreateContainerOptions, StopContainerOptions,
        UpdateContainerOptions,
    },
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
            image: c.image.clone().expect("Container without an image").clone(),
            state: c.state.clone(),
            status: c.status.clone(),
            ports: c
                .ports
                .clone()
                .unwrap_or(vec![])
                .into_iter()
                .map(|p| p.into())
                .collect(),
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
                        .map(|mapping| mapping.to_bollard_map())
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

    if let Some(restart_policy) = &req.restart_policy {
        docker_client
            .update_container(
                &req.name,
                UpdateContainerOptions::<String> {
                    restart_policy: Some(restart_policy.clone().into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
    }

    Json(ContainerCreateResponse {
        id: res.id,
        warnings: res.warnings,
    })
}

#[post("/container/start", format = "json", data = "<req>")]
pub async fn start_container(req: Json<ContainerStartRequest>) -> Json<ContainerStartResponse> {
    let docker_client = get_client().unwrap();

    docker_client
        .start_container::<String>(&req.name, None)
        .await
        .unwrap();

    Json(ContainerStartResponse { ok: true })
}

#[post("/container/stop", format = "json", data = "<req>")]
pub async fn stop_container(req: Json<ContainerStopRequest>) -> Json<ContainerStopResponse> {
    let docker_client = get_client().unwrap();

    docker_client
        .stop_container(
            &req.name,
            match req.stop_timeout {
                None => None,
                Some(stop_timeout) => Some(StopContainerOptions { t: stop_timeout }),
            },
        )
        .await
        .unwrap();

    Json(ContainerStopResponse { ok: true })
}

#[post("/container/remove", format = "json", data = "<req>")]
pub async fn remove_container(req: Json<ContainerRemoveRequest>) -> Json<ContainerRemoveResponse> {
    let docker_client = get_client().unwrap();

    docker_client
        .remove_container(&req.name, None)
        .await
        .unwrap();

    Json(ContainerRemoveResponse { ok: true })
}

fn get_client() -> Result<Docker, BollardError> {
    Ok(Docker::connect_with_local_defaults()?)
}
