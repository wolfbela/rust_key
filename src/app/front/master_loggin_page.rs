use iced::widget::{button, container, row, text_input, Button};

use super::Message;

#[allow(dead_code)]
pub fn master_login_view(password_entered: &String) -> iced::Element<'static, Message> {
    let master_password_input_box = text_input(
        "my password",
        password_entered.as_str(),
        Message::MasterPasswordChange,
    )
    .width(iced::Length::FillPortion(2))
    .password();

    let submit_button = button("Login")
        .width(iced::Length::FillPortion(1))
        .on_press(Message::SubmitMasterPasswordPress);

    row!()
        .push(master_password_input_box)
        .push(submit_button)
        .into()
}
