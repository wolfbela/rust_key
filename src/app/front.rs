pub mod login;
pub mod tabs;

use iced::Sandbox;
use tabs::Tabs;

#[derive(Debug, Default)]
pub struct PasswordManager {
    tabs: Tabs,
}

#[derive(Debug, Clone)]
pub enum Message {
    MasterPasswordInputChange(String),
    SubmitMasterPassword,
    LoginNameChange(String),
    LoginPasswordChange(String),
    AddLogin,
    RemoveLogin,
}

impl Sandbox for PasswordManager {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("RustKey")
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
