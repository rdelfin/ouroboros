#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
mod structs;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/docker", routes![routes::docker::list])
}
