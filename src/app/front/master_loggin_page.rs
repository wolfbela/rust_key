use super::Message;
use iced::widget::{button, container, row, text_input};
use std::path::Path;

const MASTER_PASSWORD_FILE_PATH: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey.json";

fn register(
    password_entered: &str,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
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
fn login(
    password_entered: &str,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let master_password_input_box = container(
        text_input(
            "my password",
            password_entered,
            Message::MasterPasswordChange,
        )
        .width(iced::Length::Units(400))
        .password(),
    )
    .height(iced::Length::Fill)
    .width(iced::Length::FillPortion(1))
    .center_x()
    .center_y();

    let submit_button = container(
        button("Login")
            .width(iced::Length::Units(100))
            .on_press(Message::SubmitMasterPasswordPress),
    )
    .height(iced::Length::Fill)
    .width(iced::Length::FillPortion(1))
    .center_x()
    .center_y();

    row!()
        .push(master_password_input_box)
        .push(submit_button)
        .into()
}

pub fn master_login_view(
    password_entered: &str,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    match Path::new(MASTER_PASSWORD_FILE_PATH).exists() {
        true => login(password_entered),
        false => register(password_entered),
    }
}
