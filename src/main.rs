use iced::widget::{button, column, text, Column, pick_list, Text};
use iced::Center;
use serde::Deserialize;


pub fn main() -> iced::Result {
    iced::run("Rustern-battle", App::update, App::view)
}

struct App {
    enemies: Enemies,
    info: String,
    selected_enemy: Option<Enemy>
}

#[derive(Debug, Clone, Deserialize)]
struct Enemies {
    enemies: Vec<Enemy>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Enemy {
    name: String,
    hp: usize,
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
        let path = std::env::var("RUSTERN_FILE_PATH").unwrap();
        let yaml_contents = std::fs::read_to_string(path).unwrap();
        let enemies: Enemies = serde_yaml::from_str(&yaml_contents).unwrap();

        Self {
            enemies: enemies,
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
        for enemy in &self.enemies.enemies {
            column = column.push(enemy.name.as_str());
        }
        let pick_list = pick_list(
            self.enemies.enemies.clone(),
            self.selected_enemy.clone(),
            Message::EnemySelected
        );
        column = column.push(pick_list);
        column.into()
    }
}