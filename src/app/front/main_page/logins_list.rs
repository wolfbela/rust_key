use crate::app::front::{Login, Message};
use iced::widget::{column, container, text};

pub fn login_view_cell(
    login: &Login,
) -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let name_login = text(&login.name);
    container(column![name_login].width(iced::Length::Units(300))).into()
}
