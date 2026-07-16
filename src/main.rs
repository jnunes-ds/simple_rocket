mod models;
mod services;

#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use serde::Serialize;
use models::client::Client;
use services::client_service;

#[get("/clients")]
fn clients() -> Template {
    let clients = client_service::get_clients();
    Template::render("clients", context! {
        clients: &clients
    })
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, clients])
        .attach(Template::fairing())
}