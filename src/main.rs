#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
