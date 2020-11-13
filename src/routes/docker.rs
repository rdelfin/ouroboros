use crate::response::Container;
use bollard::Docker;
use rocket::response::content;

#[get("/list")]
pub async fn list() -> content::Json<String> {
    let docker_client = Docker::connect_with_local_defaults().unwrap();

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
