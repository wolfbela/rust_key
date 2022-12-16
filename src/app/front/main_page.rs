use super::Message;
use crate::app::back::login_gestion::login_storing::Login;
use iced::widget::{button, column, row, text_input};

pub fn main_page_view(
    logins: &Vec<Login>,
    new_login_name: &String,
    new_login_username: &String,
    new_login_password: &String,
    adding_password: bool,
) -> iced::Element<'static, Message> {
    dbg!(logins);
    let add_login_button = button("add login")
        .width(iced::Length::Units(125))
        .on_press(Message::AddLoginPress);

    let login_gestion = row!().push(add_login_button);

    match adding_password {
        true => {
            let name = text_input("Name", &new_login_name, Message::PasswordChange);
            let submit_button = button("Submit")
                .width(iced::Length::FillPortion(1))
                .on_press(Message::RegisterNewLoginPress(
                    new_login_password.to_string(),
                    new_login_username.to_string(),
                ));
            column![name, submit_button].spacing(20).into()
        }
        false => column![login_gestion].spacing(20).into(),
    }
}
