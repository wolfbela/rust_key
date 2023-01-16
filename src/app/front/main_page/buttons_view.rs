use crate::app::front::Message;
use iced::widget::{button, column, text_input};

pub fn adding_login(
    new_login_name: &str,
    new_login_username: &str,
    new_login_password: &str,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let name = text_input("Name", new_login_name, Message::LoginNameChange);
    let username = text_input("username", new_login_username, Message::LoginUsernameChange);
    let password = text_input("password", new_login_password, Message::PasswordChange);
    let submit_button = button("Submit")
        .width(iced::Length::FillPortion(1))
        .on_press(Message::RegisterNewLoginPress(
            new_login_name.to_string(),
            new_login_password.to_string(),
            new_login_username.to_string(),
        ));

    column![name, username, password, submit_button]
        .spacing(20)
        .into()
}
