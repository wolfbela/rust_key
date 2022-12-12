use iced::widget::row;

use crate::app::back::login_gestion::password_storing::Login;

use super::Message;

pub fn main_page_view(_logins: &Vec<Login>) -> iced::Element<'static, Message> {
    row!().into()
}
