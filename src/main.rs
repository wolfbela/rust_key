mod app;

use app::front::PasswordManager;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    let settings = Settings::default();
    settings.exit_on_close_request = false;
    PasswordManager::run(settings)
}
