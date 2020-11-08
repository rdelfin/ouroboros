#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod docker;

#[get("/")]
async fn index() -> String {
    let containers = docker::list().await.unwrap();
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
