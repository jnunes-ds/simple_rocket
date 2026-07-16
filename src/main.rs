mod models;
mod services;
mod controllers;

#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use controllers::{home_controller, clients_controller};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            home_controller::index,
            clients_controller::index
        ])
        .attach(Template::fairing())
}