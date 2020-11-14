use crate::structs::{Container, ImageCreateRequest, ImageCreateResponse};
use bollard::{errors::Error as BollardError, image::CreateImageOptions, Docker};
use rocket::futures::StreamExt;
use rocket_contrib::json::Json;

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

fn get_client() -> Result<Docker, BollardError> {
    Ok(Docker::connect_with_local_defaults()?)
}
