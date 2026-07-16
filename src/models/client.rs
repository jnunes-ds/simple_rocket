use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Client {
    pub id: u32,
    pub name: String,
    pub cpf: String,
}