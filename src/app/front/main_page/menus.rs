use iced::widget::{button, column, container};

use crate::app::front::Message;

pub fn menus() -> iced::Element<'static, Message, iced::Renderer<crate::app::style::theme::Theme>> {
    let password = button("passwords")
        .style(crate::app::style::button_theme::Button::Menus)
        .on_press(Message::AddLoginPress);

    container(column!().push(password))
        .width(iced::Length::FillPortion(1))
        .height(iced::Length::Fill)
        .padding(iced::Padding {
            top: 5,
            right: 5,
            bottom: 30,
            left: 10,
        })
        .into()
}
