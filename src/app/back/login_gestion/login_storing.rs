use std::collections::HashMap;

#[derive(Debug)]
pub struct Login {
    _login: String,
    _password: String,
    _associated_websites: String,
}

#[derive(Debug)]
struct Logins {
    _logins: HashMap<String, Login>,
}

enum Error {
    _CouldNotAddLogin,
    _CouldNotRemoveLogin,
    _LoginNotFound,
}

impl Logins {
    #[allow(dead_code)]
    pub fn add_login() -> Result<(), Error> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn remove_login() -> Result<(), Error> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn load_logins() -> Result<(), Error> {
        todo!()
    }
}
