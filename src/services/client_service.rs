use crate::models::client::Client;

pub fn get_clients() -> Vec<Client> {
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