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

pub fn get_client_by_id(id: u32) -> Result<Client, String> {
    match get_clients().iter().find(|c| c.id == id) {
        Some(client) => Ok(Client {
            id: client.id,
            name: client.name.clone(),
            cpf: client.cpf.clone(),
        }),
        None => Err("Client not found".to_string()),
    }
    
}

pub fn create_client(name: String, cpf: String) -> bool {
    if name == "" || cpf == "" {
        return false
    }
    true
}


pub fn update_client(id: u32, name: String, cpf: String) -> bool {
    let client = get_client_by_id(id);
    if client.is_err() || name.trim() == "" || cpf.trim() == "" {
        return false;
    }
    true
}

pub fn delete_client_by_id(id: u32) -> Result<bool, String> {
    match get_client_by_id(id) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }

}