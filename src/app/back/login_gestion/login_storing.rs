#[derive(Debug)]
pub struct Login {
    _name: String,
    _login: String,
    _password: String,
    _associated_websites: String,
}

impl Login {
    pub fn new() -> Login {
        Login {
            _name: String::from(""),
            _login: String::from(""),
            _password: String::from(""),
            _associated_websites: String::from(""),
        }
    }
}

#[derive(Debug)]
pub struct Logins {
    pub logins: Vec<Login>,
}

pub enum Error {
    _CouldNotAddLogin,
    _CouldNotRemoveLogin,
    _LoginNotFound,
}

impl Default for Logins {
    fn default() -> Self {
        Logins { logins: vec![] }
    }
}

impl Logins {
    pub fn new() -> Logins {
        Logins { logins: vec![] }
    }

    #[allow(dead_code)]
    pub fn add_login(self) -> Result<(), Error> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn remove_login(self) -> Result<(), Error> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn load_logins(self) -> Result<(), Error> {
        todo!()
    }
}
