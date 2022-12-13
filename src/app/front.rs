pub mod main_page;
pub mod master_loggin_page;

use crate::app::back::login_gestion::password_storing::Login;
use iced::{Element, Sandbox};

use super::back::master_login::verify_master_password;
use crate::app::back::master_login::register_master_password;
use main_page::main_page_view;
use master_loggin_page::master_login_view;

#[derive(Debug)]
pub struct PasswordManager {
    master_password: String,
    passwords: Vec<Login>,
    is_logged: bool,
    error_master_password: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    MasterPasswordChange(String),
    SubmitMasterPasswordPress,
    RegisterMasterPassword,
    _AddLoginPress,
    _RemoveLogin,
    _LoginChange(String),
    _PasswordChange(String),
}

impl Sandbox for PasswordManager {
    type Message = Message;

    fn new() -> Self {
        PasswordManager {
            master_password: String::from(""),
            passwords: vec![],
            is_logged: false,
            error_master_password: false,
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
            _ => todo!(),
            // Message::LoginChange(_) => todo!(),
            // Message::PasswordChange(_) => todo!(),
            // Message::AddLoginPress => todo!(),
            // Message::RemoveLogin => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.is_logged {
            true => main_page_view(&self.passwords),
            false => master_login_view(&self.master_password.as_str()),
        }
    }
}
