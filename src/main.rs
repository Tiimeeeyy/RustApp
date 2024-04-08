use crate::user_interface::MyApp;

mod email_sender;
mod user_interface;

fn main() -> iced::Result {
    MyApp::MyApp::run(iced::Settings::default())
}
