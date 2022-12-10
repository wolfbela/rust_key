mod app;

use app::front::PasswordManager;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    PasswordManager::run(Settings::default())
}
