use iced::widget::{row, Text};

use crate::app::back::login_gestion::password_storing::Password;

use super::Message;

pub fn main_page_view(logins: &Vec<Password>) -> iced::Element<'static, Message> {
    row!().into()
}
