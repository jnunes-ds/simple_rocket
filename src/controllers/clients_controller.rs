use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::request::FlashMessage;
use rocket_dyn_templates::{context, Template};
use crate::dto::client_dto::ClientDTO;
use crate::services::client_service;


#[get("/clients")]
pub fn index() -> Template {
    let clients = client_service::get_clients();
    Template::render("clients/index", context! {
        clients: &clients
    })
}

#[get("/clients/new")]
pub fn new(flash: Option<FlashMessage<'_>>) -> Template {
    let mut error = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            error = msg.message().to_string();
        }
    }
    Template::render("clients/new", context! { error: &error })
}

#[post("/clients/create", data = "<client_dto_form>")]
pub fn create(client_dto_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client_dto = client_dto_form.into_inner();

    // Salvar os dados no db
    println!("Client name: {}", client_dto.name);
    println!("Client cpf: {}", client_dto.cpf);

    if client_dto.name != "" || client_dto.cpf != "" {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to("/clients/new"),
            "Invalid data"
        ))
    }

}