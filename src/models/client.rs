use rocket::serde::Serialize;

#[derive(Serialize, FromForm)]
pub struct Client {
    pub id: u32,
    pub name: String,
    pub cpf: String,
}