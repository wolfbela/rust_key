use super::Message;
use iced::widget::{button, row, text_input};
use std::path::Path;

const MASTER_PASSWORD_FILE_PATH: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey.json";

fn register(password_entered: &str) -> iced::Element<'static, Message> {
    let master_password_input_box = text_input(
        "my password",
        password_entered,
        Message::MasterPasswordChange,
    )
    .width(iced::Length::FillPortion(2))
    .password();

    let register_button = button("Register")
        .width(iced::Length::FillPortion(1))
        .on_press(Message::RegisterMasterPassword);

    row!()
        .push(master_password_input_box)
        .push(register_button)
        .into()
}

#[allow(dead_code)]
fn login(password_entered: &str) -> iced::Element<'static, Message> {
    let master_password_input_box = text_input(
        "my password",
        password_entered,
        Message::MasterPasswordChange,
    )
    .width(iced::Length::FillPortion(2))
    .password();

    let submit_button = button("Login")
        .width(iced::Length::FillPortion(1))
        .on_press(Message::SubmitMasterPasswordPress);

    row!()
        .push(master_password_input_box)
        .push(submit_button)
        .into()
}

pub fn master_login_view(password_entered: &str) -> iced::Element<'static, Message> {
    match Path::new(MASTER_PASSWORD_FILE_PATH).exists() {
        true => login(password_entered),
        false => register(password_entered),
    }
}
