use iced::{button, Button, Column, Element, Text, Settings, Sandbox};

#[derive(Default)]
struct MainWindow {
    select_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectPressed,
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Jackbox Custom Content")
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
        .push(
            Button::new(&mut self.select_button, Text::new("Select"))
            .on_press(Message::SelectPressed)
        ).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectPressed => {
                println!("Select pressed.");
            }
        }
    }
}

fn main() -> iced::Result {
    println!("Starting window...");
    MainWindow::run(Settings::default())
}
