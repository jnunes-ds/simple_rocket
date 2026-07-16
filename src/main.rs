#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use serde::Serialize;

#[derive(Serialize)]
struct Client {
    id: u32,
    name: String,
    cpf: String,
}

fn get_clients() -> Vec<Client> {
    vec![
        Client {
            id: 1,
            name: "John Doe".to_string(),
            cpf: "123.456.789-00".to_string(),
        },
        Client {
            id: 2,
            name: "Jane Smith".to_string(),
            cpf: "987.654.321-11".to_string(),
        },
        Client {
            id: 3,
            name: "Alice Johnson".to_string(),
            cpf: "456.789.123-22".to_string(),
        },
    ]
}

#[get("/clients")]
fn clients() -> Template {
    let clients = get_clients();
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