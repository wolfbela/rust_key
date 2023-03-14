use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub name: String,
    pub username: String,
    pub password: String,
    pub associated_websites: String,
}

impl Default for Login {
    fn default() -> Self {
        Self {
            name: String::from(""),
            username: String::from(""),
            password: String::from(""),
            associated_websites: String::from(""),
        }
    }
}
