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
    let error = flash_error(flash);
    Template::render("clients/new", context! { error: &error })
}

#[post("/clients/create", data = "<client_dto_form>")]
pub fn create(client_dto_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client_dto = client_dto_form.into_inner();

    // Salvar os dados no db
    println!("Client name: {}", client_dto.name);
    println!("Client cpf: {}", client_dto.cpf);

    if client_service::create_client(client_dto.name, client_dto.cpf) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to("/clients/new"),
            "Invalid data"
        ))
    }

}

#[get("/clients/<id>/edit")]
pub fn edit(id: u32, flash: Option<FlashMessage<'_>>) -> Template {
    let error = flash_error(flash);

    match client_service::get_client_by_id(id) {
        Ok(client) => Template::render("clients/edit", context! {
            client: &client,
            error: &error
        }),
        Err(error) => Template::render("clients/edit", context! {
            error: &error
        })
    }
}

#[post("/clients/<id>/update", data = "<client_dto_form>")]
pub fn update(id: u32, client_dto_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client_dto = client_dto_form.into_inner();

    // Salvar os dados no db
    println!("Client name: {}", client_dto.name);
    println!("Client cpf: {}", client_dto.cpf);

    if client_service::update_client(id, client_dto.name, client_dto.cpf) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clients/{}/edit", id), ),
            "Invalid data"
        ))
    }

}

#[get("/clients/<id>/delete")]
pub fn delete(id: u32) -> Result<Redirect, Flash<Redirect>> {
    match client_service::delete_client_by_id(id) {
        Ok(_) => Ok(Redirect::to("/clients")),
        Err(err) => Err(Flash::error(
            Redirect::to(format!("/clients/{}/edit", id), ),
            &err
        ))
    }
}


fn flash_error(flash: Option<FlashMessage<'_>>) -> String {
    let mut error = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            error = msg.message().to_string();
        }
    };

    error
}