pub mod buttons_view;
pub mod header;
pub mod logins_list;

use super::Message;
use crate::app::back::login_gestion::login_storing::Login;
use crate::app::style::theme::Theme;
use iced::widget::{button, column, scrollable};

use buttons_view::adding_login;
use logins_list::login_view_cell;

fn classic_display(
    logins: &Vec<Login>,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let add_login_button = button("add login")
        .width(iced::Length::Units(125))
        .on_press(Message::AddLoginPress);

    let list_of_login = logins
        .into_iter()
        .map(|login| login_view_cell(login))
        .collect::<Vec<iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>>>>(
        );

    // column![add_login_button, list_of_login].spacing(20).into()
    scrollable(
        column(list_of_login)
            .push(add_login_button)
            .height(iced::Length::Fill)
            .spacing(10),
    )
    .into()
}

pub fn main_page_view(
    logins: &Vec<Login>,
    new_login_name: &str,
    new_login_username: &str,
    new_login_password: &str,
    adding_password: bool,
) -> iced::Element<'static, Message, iced::Renderer<Theme>> {
    match adding_password {
        true => adding_login(new_login_name, new_login_username, new_login_password),
        false => classic_display(logins),
    }
}
