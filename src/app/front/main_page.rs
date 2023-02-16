pub mod buttons_view;
pub mod header;
pub mod logins_list;
pub mod menus;

use super::Message;
use crate::app::back::login_gestion::login_storing::Login;
use crate::app::style::theme::Theme;
use iced::widget::{column, container, row, scrollable, vertical_rule};
use menus::menus;

use buttons_view::adding_login;
use header::header;
use logins_list::login_view_cell;

fn list_login(
    logins: &Vec<Login>,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let list_of_login = logins
        .into_iter()
        .map(|login| login_view_cell(login))
        .collect::<Vec<iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>>>>();

    container(
        column!()
            .push(header())
            .push(scrollable(
                column(list_of_login).height(iced::Length::Fill).spacing(10),
            ))
            .width(iced::Length::FillPortion(3)),
    )
    .style(crate::app::style::container::Container::NotSelectable)
    .width(iced::Length::FillPortion(3))
    .height(iced::Length::Fill)
    .padding(iced::Padding {
        top: 5,
        right: 30,
        bottom: 30,
        left: 30,
    })
    .into()
}

fn classic_display(
    logins: &Vec<Login>,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    row!()
        .push(menus())
        .push(container(vertical_rule(1)))
        .push(list_login(logins))
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
