use super::Message;
use crate::app::back::login_gestion::login_storing::Login;
use iced::widget::{button, column, row};

pub fn main_page_view(_logins: &Vec<Login>) -> iced::Element<'static, Message> {
    let add_login_button = button("add login")
        .width(iced::Length::Units(125))
        .on_press(Message::AddLoginPress);

    let login_gestion = row!().push(add_login_button);

    column![login_gestion].spacing(20).into()
}
