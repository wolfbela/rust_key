pub mod master_login_page;

use crate::app::back::login_gestion::password_storing::Password;

#[derive(Debug)]
enum Tab {
    StartPage { master_password: String },
    MainPage { passwords: Vec<Password> },
}

#[derive(Debug, Default)]
pub struct Tabs {
    tabs: Vec<Tab>,
    current: usize,
}

pub enum TabsMessage {
    Login(String, bool),
}
