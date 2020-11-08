#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod docker;

#[get("/")]
async fn index() -> &'static str {
    docker::list().await.unwrap();
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
