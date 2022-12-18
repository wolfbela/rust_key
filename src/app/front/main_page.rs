use super::Message;
use crate::app::back::login_gestion::login_storing::Login;
use iced::widget::{button, column, row, text_input};

fn adding_login(
    new_login_name: &str,
    new_login_username: &str,
    new_login_password: &str,
) -> iced::Element<'static, Message> {
    let name = text_input("Name", new_login_name, Message::LoginNameChange);
    let username = text_input("username", new_login_username, Message::LoginUsernameChange);
    let password = text_input("username", new_login_password, Message::PasswordChange);
    let submit_button = button("Submit")
        .width(iced::Length::FillPortion(1))
        .on_press(Message::RegisterNewLoginPress(
            new_login_password.to_string(),
            new_login_username.to_string(),
        ));
    column![name, username, password, submit_button]
        .spacing(20)
        .into()
}

fn classic_display() -> iced::Element<'static, Message> {
    let add_login_button = button("add login")
        .width(iced::Length::Units(125))
        .on_press(Message::AddLoginPress);

    let login_gestion = row!().push(add_login_button);
    column![login_gestion].spacing(20).into()
}

pub fn main_page_view(
    logins: &Vec<Login>,
    new_login_name: &str,
    new_login_username: &str,
    new_login_password: &str,
    adding_password: bool,
) -> iced::Element<'static, Message> {
    match adding_password {
        true => adding_login(new_login_name, new_login_username, new_login_password),
        false => classic_display(),
    }
}
