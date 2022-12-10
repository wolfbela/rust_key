pub mod main_page;
pub mod master_loggin_page;

use crate::app::back::login_gestion::password_storing::Password;
use iced::Sandbox;

#[derive(Debug)]
pub struct PasswordManager {
    master_login_page: Tab,
    main_page: Tab,
    is_logged: bool,
}

#[derive(Debug)]
enum Tab {
    StartPage { master_password: String },
    MainPage { passwords: Vec<Password> },
}

#[derive(Debug, Clone)]
pub enum Message {
    MasterPasswordChange(String),
    SubmitMasterPasswordPress,
    AddLoginPress,
    RemoveLogin,
    LoginChange(String),
    PasswordChange(String),
}

impl Sandbox for PasswordManager {
    type Message = Message;

    fn new() -> Self {
        Self::new()
    }

    fn title(&self) -> String {
        String::from("RustKey")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MasterPasswordChange(_) => todo!(),
            Message::SubmitMasterPasswordPress => todo!(),
            Message::AddLoginPress => todo!(),
            Message::RemoveLogin => todo!(),
            Message::LoginChange(_) => todo!(),
            Message::PasswordChange(_) => todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
