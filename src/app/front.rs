pub mod login;
pub mod login_page;

use iced::Sandbox;

pub struct PasswordManager {
    login: String,
    password: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddPassword(String),
    AddLogin(String),
    Submit,
}

impl Sandbox for PasswordManager {
    type Message = Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
