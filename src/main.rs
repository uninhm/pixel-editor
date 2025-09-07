use iced;
use iced::widget::{button, column, text, row, text_input, Column};
use iced::Length::Fill;

#[derive(Default)]
struct App {
    search_input_string: String,
}

#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    SearchButtonPressed,
}

impl App {
    fn view(&self) -> Column<'_, Message> {
        column![
            row![
                text_input("Search...", &self.search_input_string)
                    .on_input(Message::SearchInputChanged)
                    .width(Fill),
                button("Search")
                    .on_press(Message::SearchButtonPressed)
            ],
            text(format!("You searched for: {}", self.search_input_string))
                .width(Fill)
        ]
    }
    
    fn update(&mut self, message: Message) {
        match message {
            Message::SearchInputChanged(s) => {
                self.search_input_string = s;
            }
            Message::SearchButtonPressed => {
                println!("Search button pressed with query: {}", self.search_input_string);
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("Pixel Editor", App::update, App::view)
}
