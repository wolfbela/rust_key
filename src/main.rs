use iced::{Sandbox, Settings};

mod app;

use app::front::PasswordManager;

fn main() {
    PasswordManager::run(Settings::default());
}
