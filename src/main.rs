mod app;

use app::front::PasswordManager;
use iced::{Application, Settings};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.exit_on_close_request = false;
    PasswordManager::run(settings)
}
