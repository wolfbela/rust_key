pub mod main_page;
pub mod master_loggin_page;

use crate::app::back::{login_gestion::login_storing::Login, write_logins_into_file};
use iced::{window, Application, Command, Element, Subscription};

use super::back::master_login::verify_master_password;
use crate::app::back::master_login::register_master_password;
use main_page::main_page_view;
use master_loggin_page::master_login_view;

const PATH_OF_LOGINS_FILE: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey-logins.json";

#[derive(Debug)]
pub struct NewLogin {
    pub new_login_password: String,
    pub new_login_username: String,
    pub new_login_name: String,
}

#[derive(Debug)]
pub struct PasswordManager {
    master_password: String,
    logins: Vec<Login>,
    is_logged: bool,
    new_login: NewLogin,
    error_master_password: bool,
    encrytion_key: [u8; 32],
    adding_login: bool,
    should_exit: bool,
    _removing_login: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    MasterPasswordChange(String),
    SubmitMasterPasswordPress,
    RegisterMasterPassword,
    AddLoginPress,
    RegisterNewLoginPress(String, String, String),
    _RemoveLogin,
    PasswordChange(String),
    LoginNameChange(String),
    OnEvent(iced::Event),
    LoginUsernameChange(String),
}

impl Application for PasswordManager {
    type Message = Message;

    fn new(_flags: ()) -> (PasswordManager, Command<Message>) {
        (
            PasswordManager {
                master_password: String::from(""),
                logins: vec![],
                is_logged: false,
                new_login: NewLogin {
                    new_login_password: String::from(""),
                    new_login_username: String::from(""),
                    new_login_name: String::from(""),
                },
                error_master_password: false,
                encrytion_key: [0u8; 32],
                adding_login: false,
                should_exit: false,
                _removing_login: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("RustKey")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::MasterPasswordChange(new_master_password) => {
                self.master_password = new_master_password;
            }
            Message::SubmitMasterPasswordPress => {
                match verify_master_password(&self.master_password) {
                    true => {
                        self.is_logged = true;
                        self.error_master_password = false;
                    }
                    false => {
                        self.error_master_password = true;
                    }
                }
                self.master_password = String::from("");
            }
            Message::RegisterMasterPassword => {
                register_master_password(&self.master_password);
                self.master_password = String::from("");
            }
            Message::AddLoginPress => {
                self.adding_login = true;
            }
            Message::PasswordChange(new_password) => {
                self.new_login.new_login_password = new_password;
            }
            Message::LoginUsernameChange(new_username) => {
                self.new_login.new_login_username = new_username;
            }
            Message::LoginNameChange(new_name) => {
                self.new_login.new_login_name = new_name;
            }
            Message::RegisterNewLoginPress(name, username, password) => {
                self.logins.push(Login {
                    name: name,
                    username: username,
                    password: password,
                    associated_websites: String::from(""),
                });

                /*
                reset new login value before closing the adding form to reset its feilds.
                */
                self.new_login.new_login_name = String::from("");
                self.new_login.new_login_username = String::from("");
                self.new_login.new_login_password = String::from("");
                self.adding_login = false;
            }
            Message::OnEvent(event) => {
                /*
                We are looking if we clicked on the closing button.

                At this time, we can save the logins in a file and then live the
                */
                if iced::Event::Window(window::Event::CloseRequested) == event {
                    dbg!("Sould exit know");
                    match write_logins_into_file(
                        serde_json::to_string(&self.logins).unwrap().as_str(),
                        PATH_OF_LOGINS_FILE,
                        &self.encrytion_key,
                    ) {
                        Ok(_) => dbg!("Could write the content in a file"),
                        Err(_) => dbg!("Could not write the content in a file"),
                    };
                    self.should_exit = true;
                }
            }
            Message::_RemoveLogin => todo!(),
        }

        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::OnEvent)
    }

    /*
    This function exist in the default application trait.
    By default, it's at false (not exiting application).

    Here, we're ovewriting the implementation and now, it's depend of the `should_exit` attribute wich pass to true
    with clicking on the closing button.
    */
    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.is_logged {
            /*
            Self passwords is encrypted at the beginning, it will be decrypted after the user is logged in.
            So it need to be a mutable variable.
            */
            true => main_page_view(
                &self.logins,
                self.new_login.new_login_name.as_str(),
                self.new_login.new_login_username.as_str(),
                self.new_login.new_login_password.as_str(),
                self.adding_login,
            ),
            false => master_login_view(&self.master_password.as_str()),
        }
    }

    type Executor = iced::executor::Default;

    type Theme = iced::Theme;

    type Flags = ();
}
