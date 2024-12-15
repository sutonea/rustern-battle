use iced::widget::{button, column, text, Column, pick_list, Text};
use iced::Center;

pub fn main() -> iced::Result {
    iced::run("Rustern-battle", App::update, App::view)
}

struct App {
    enemies: Vec<Enemy>,
    info: String,
    selected_enemy: Option<Enemy>
}

#[derive(Debug, Clone, PartialEq)]
struct Enemy {
    name: String,
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
enum Message {
    EnemySelected(Enemy),
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
            info: "".to_string(),
            selected_enemy: None,
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::EnemySelected(enemy) => {
                self.info = enemy.name.clone();
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let mut column = Column::new();
        column = column.push(self.info.as_str());
        for enemy in &self.enemies {
            column = column.push(enemy.name.as_str());
        }
        let pick_list = pick_list(
            self.enemies.clone(),
            self.selected_enemy.clone(),
            Message::EnemySelected
        );
        column = column.push(pick_list);
        column.into()
    }
}