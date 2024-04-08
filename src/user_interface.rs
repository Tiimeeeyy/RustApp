/// Importing the important crates used in this class
use iced::futures::select_biased;
use iced::widget::text::State;
use iced::{
    executor, widget::Button, widget::Column, widget::Container, widget::Row, widget::Scrollable,
    widget::Text, widget::TextInput, Alignment, Application, Command, Element, Length, Renderer,
    Settings, widget::Checkbox,
};
use iced::window::Action::ChangeIcon;
use rusqlite::{params, Connection, Result};

/// This struct is the "creator"  (as a field) with attributes: id, name, email, followers and UI States
pub struct Creator<´a, Message + ´a + Clone> {
    id: i32,
    name: String,
    email: String,
    followers: i32,
    name_state: TextInput<´a, Name>::State,
    email_state: TextInput::State,
    followers_state: TextInput::State,
    selected: bool,
    checkbox_state: iced::widget::Checkbox::State,
}

/// This struct represent the app
pub struct MyApp {
    creators: Vec<Creator>,
    save_button: Button::State,
}

/// Enum for handling different types of messages / events in the application
#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(usize, String),
    EmailChanged(usize, String),
    FollowersChanged(usize, String),
    CreatorSelected(usize, bool),
    DeleteCreator(usize),
    Save,
}

/// Implementation for MyApp
impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let creators = fetch_creators().unwrap();
        (
            MyApp {
                creators,
                save_button: Button::State::new(),
            },
            Command::none(),
        )
    }
    /// Setting the title for the application
    fn title(&self) -> String {
        String::from("Creator Mailer")
    }

    /// Function to handle updates based on certain events
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NameChanged(index, new_name) => {
                self.creators[index].name = new_name;
            }
            Message::EmailChanged(index, new_email) => {
                self.creators[index].email = new_email;
            }
            Message::FollowersChanged(index, new_followers) => {
                self.creators[index].followers = new_followers.parse().unwrap_or(0);
            }
            Message::Save => {
                save_creators(&self.creators).unwrap();
            }
            Message::CreatorSelected(index, selected) => {
                self.creators[index].selected = selected;
            }
        }
        Command::none()
    }
    /// This is the view function, it includes all the inputs and text boxes used for the database controller
    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut content = Column::new();
        for (i, creator) in self.creators.iter_mut().enumerate() {
            let row = Row::new()
                .push(
                    TextInput::new(&mut creator.name_state, "Name")
                        .value(&creator.name)
                        .on_change(move |name| Message::NameChanged(i, name)),
                )
                .push(
                    TextInput::new(&mut creator.email_state, "Email")
                        .value(&creator.email)
                        .on_change(move |email| Message::EmailChanged(i, email)),
                )
                .push(
                    TextInput::new(&mut creator.followers_state, "Followers")
                        .value(&creator.followers.to_string())
                        .on_change(move |followers| Message::FollowersChanged(i, followers)),
                )
                .push(
                    Checkbox::new(creator.selected, move |checked| {
                        Message::CreatorSelected(i, checked)
                    })
                        .State(&mut creator.checkbox_state),
                );

            content = content.push(row);
        }
        content = content.push(
            Button::new(&mut self.save_button)
                .label(Text::new("Save"))
                .on_press(Message::Save),
        );
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

/// Function to fetch creators from the database
fn fetch_creators() -> Result<Vec<Creator>> {
    let conn = Connection::open("my_db.db")?;
    let mut stmt = conn.prepare("SELECT Id, Name, Email, Followers FROM Creators")?;
    let rows = stmt.query_map(params![], |row| {
        Ok(Creator {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            followers: row.get(3)?,
            name_state: TextInput::State::new(),
            email_state: TextInput::State::new(),
            followers_state: TextInput::State::new(),
            selected: false,
            checkbox_state: (),
        })
    })?;
    Ok(rows.collect::<Result<_>>()?)
}

/// Function to save creators to the database
fn save_creators(creators: &[Creator]) -> Result<()> {
    let conn = Connection::open("my_db.db")?;
    for creator in creators {
        conn.execute(
            "UPDATE Creators SET Name = ?, Email = ?, Followers = ? WHERE Id = ?",
            params![creator.name, creator.email, creator.followers, creator.id],
        )?;
    }
    Ok(())
}

/// Function to insert a new creator into the database
fn insert_creator(creator: &Creator) -> Result<()> {
    let conn = Connection::open("database.db")?;
    conn.execute(
        "INSERT INTO Creators (Name, Email, Followers) VALUES (?, ?, ?)",
        params![creator.name, creator.email, creator.followers],
    )?;
    Ok(())
}
