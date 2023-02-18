use crate::app::front::Message;
use iced::widget::{button, container};

pub fn header() -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>>
{
    let add_login_button = button("add login")
        .width(iced::Length::Units(125))
        .on_press(Message::AddLoginPress);

    container(add_login_button)
        .height(iced::Length::Units(75))
        .width(iced::Length::Fill)
        .into()
}
