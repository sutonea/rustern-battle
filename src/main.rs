use iced::widget::{button, column, text, Column};
use iced::Center;

pub fn main() -> iced::Result {
    iced::run("Rustern-battle", App::update, App::view)
}

#[derive(Default)]
struct App {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
            .padding(20)
            .align_x(Center)
    }
}