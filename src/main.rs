use iced::widget::{button, column, text, Column, Text};
use iced::Center;

pub fn main() -> iced::Result {
    iced::run("Rustern-battle", App::update, App::view)
}

struct App {
    enemies: Vec<Enemy>,
}

struct Enemy {
    name: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    fn new() -> Self {
        Self {
            enemies: vec![
                Enemy { name: "Enemy A".to_string() },
                Enemy { name: "Enemy B".to_string() },
                Enemy { name: "Enemy C".to_string() },
            ],
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
            }
            Message::Decrement => {
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let mut column = Column::new();
        for enemy in &self.enemies {
            column = column.push(enemy.name.as_str());
        }
        column.into()
    }
}