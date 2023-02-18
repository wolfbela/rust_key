use crate::app::front::{Login, Message};
use iced::widget::{container, row, text};

pub fn login_view_cell(
    login: &Login,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let name_login = text(&login.name).width(iced::Length::FillPortion(1));
    let password_login = text(&login.password).width(iced::Length::FillPortion(1));
    let username_login = text(&login.username).width(iced::Length::FillPortion(1));
    container(row![name_login, password_login, username_login].width(iced::Length::Fill)).into()
}
