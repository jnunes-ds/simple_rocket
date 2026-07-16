use rocket_dyn_templates::{context, Template};
use crate::services::client_service;

#[get("/clients")]
pub fn index() -> Template {
    let clients = client_service::get_clients();
    Template::render("clients/index", context! {
        clients: &clients
    })
}

#[get("/clients/new")]
pub fn new() -> Template {
    Template::render("clients/new", context! {})
}