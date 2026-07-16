mod models;
mod services;
mod dto;

mod controllers {
    pub mod home_controller;
    pub mod clients_controller;
}

#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use controllers::{home_controller, clients_controller};
use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            home_controller::index,
            clients_controller::index,
            clients_controller::new,
            clients_controller::create,
            clients_controller::edit,
            clients_controller::update,
        ])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}