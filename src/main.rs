#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
mod structs;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/docker",
            routes![
                routes::docker::list,
                routes::docker::create_image,
                routes::docker::create_container,
                routes::docker::start_container,
                routes::docker::stop_container,
                routes::docker::remove_container,
            ],
        )
        .mount("/", routes![routes::misc::health])
}
