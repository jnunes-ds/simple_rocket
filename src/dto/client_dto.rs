#[derive(FromForm)]
pub struct ClientDTO {
    pub name: String,
    pub cpf: String,
}