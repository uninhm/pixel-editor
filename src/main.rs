use iced::Task;
use iced::widget::{column, text, row, text_input, Column};
use iced::Length::Fill;
use iced::keyboard;

#[derive(Default)]
struct App {
    search_input_string: String,
}

#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    FocusSearchInput,
}

impl App {
    fn view(&self) -> Column<'_, Message> {
        let searchable_items = vec!["Cheese", "Milk", "Eggs", "Butter"];
        let search_results =
            if !self.search_input_string.is_empty() {
                column(
                    searchable_items
                        .iter()
                        .filter(|item| item.to_lowercase().contains(&self.search_input_string.to_lowercase()))
                        .map(|item| text(*item).into())
                )
            } else {
                column![]
            };

        column![
            row![
                text_input("Search...", &self.search_input_string)
                    .id("search_input")
                    .on_input(Message::SearchInputChanged)
                    .width(Fill),
            ],
            search_results,
        ]
    }
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchInputChanged(s) => {
                self.search_input_string = s;
                Task::none()
            },
            Message::FocusSearchInput => {
                text_input::focus("search_input")
            }
        }
    }
    
    fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| {
            match key.as_ref() {
                keyboard::Key::Character("/") => Some(Message::FocusSearchInput),
                _ => None,
            }
        })
    }
}

fn main() -> iced::Result {
    iced::application("Pixel Editor", App::update, App::view)
        .subscription(App::subscription)
        .run()
}
