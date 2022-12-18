pub mod main_page;
pub mod master_loggin_page;

use crate::app::back::login_gestion::login_storing::Logins;
use iced::{Element, Sandbox};

use super::back::master_login::verify_master_password;
use crate::app::back::master_login::register_master_password;
use main_page::main_page_view;
use master_loggin_page::master_login_view;

#[derive(Debug)]
pub struct NewLogin {
    pub new_login_password: String,
    pub new_login_username: String,
    pub new_login_name: String,
}

#[derive(Debug)]
pub struct PasswordManager {
    master_password: String,
    passwords: Logins,
    is_logged: bool,
    new_login: NewLogin,
    error_master_password: bool,
    adding_login: bool,
    _removing_login: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    MasterPasswordChange(String),
    SubmitMasterPasswordPress,
    RegisterMasterPassword,
    AddLoginPress,
    RegisterNewLoginPress(String, String),
    _RemoveLogin,
    PasswordChange(String),
    LoginNameChange(String),
    LoginUsernameChange(String),
}

impl Sandbox for PasswordManager {
    type Message = Message;

    fn new() -> Self {
        PasswordManager {
            master_password: String::from(""),
            passwords: Logins::new(),
            new_login: NewLogin {
                new_login_password: String::from(""),
                new_login_username: String::from(""),
                new_login_name: String::from(""),
            },
            is_logged: false,
            error_master_password: false,
            adding_login: false,
            _removing_login: false,
        }
    }

    fn title(&self) -> String {
        String::from("RustKey")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MasterPasswordChange(new_master_password) => {
                self.master_password = new_master_password;
            }
            Message::SubmitMasterPasswordPress => {
                match verify_master_password(&self.master_password) {
                    true => {
                        self.is_logged = true;
                        self.error_master_password = false;
                    }
                    false => {
                        self.error_master_password = true;
                    }
                }
                self.master_password = String::from("");
            }
            Message::RegisterMasterPassword => {
                register_master_password(&self.master_password);
                self.master_password = String::from("");
            }
            Message::AddLoginPress => {
                self.adding_login = true;
            }
            Message::PasswordChange(new_password) => {
                self.new_login.new_login_password = new_password;
            }
            Message::LoginUsernameChange(new_username) => {
                self.new_login.new_login_username = new_username;
            }
            Message::LoginNameChange(new_name) => {
                self.new_login.new_login_name = new_name;
            }
            Message::RegisterNewLoginPress(_login, _password) => {}
            Message::_RemoveLogin => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.is_logged {
            /*
            Self passwords is encrypted at the beginning, it will be decrypted after the user is logged in.
            So it need to be a mutable variable.
            */
            true => main_page_view(
                &self.passwords.logins,
                self.new_login.new_login_name.as_str(),
                self.new_login.new_login_username.as_str(),
                self.new_login.new_login_password.as_str(),
                self.adding_login,
            ),
            false => master_login_view(&self.master_password.as_str()),
        }
    }
}
