use iced::window::resize;
use iced::Task;
use iced::widget::{column, text, row, text_input, Column};
use iced::Length::Fill;
use iced::keyboard;

#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    FocusSearchInput,
}

struct App {
    search_input_string: String,
    atoms: Vec<Atom>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_input_string: String::new(),
            atoms: import_csv(),
        }
    }
}

impl App {
    fn view(&self) -> Column<'_, Message> {
        let search_results =
            if !self.search_input_string.is_empty() {
                column(
                    self.atoms
                        .iter()
                        .filter(|atom| atom.contains(&self.search_input_string))
                        .map(|atom| text(atom.words.first().unwrap()).into())
                    // TODO: Display a row with the pattent on the left and the words on the right
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
        ].padding(10)
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

struct Atom {
    words: Vec<String>,

    // Pattern is stored as a bitmask. The n-th bit (lsb is 0) encodes the
    // (n%5)th pixel from the right in the (n/5)th row from the bottom.
    pattern: u32,
}

impl Atom {
    fn new(words: Vec<String>, pattern: u32) -> Self {
        Self { words, pattern }
    }
    
    // Csv format description:
    // First element is the pattern represented as a u32 as described above.
    //   Base 10 with no prefix, base 2 with "0b" prefix, or base 16 with "0x" prefix.
    // The next elements are words associated with the pattern.
    fn from_csv_record(record: &csv::StringRecord) -> Self {
        let pattern_str = &record[0];
        let radix = if pattern_str.starts_with("0b") {
            2
        } else if pattern_str.starts_with("0x") {
            16
        } else {
            10
        };
        let number_no_prefix = if radix == 10 {
            pattern_str
        } else {
            &pattern_str[2..]
        };
        let pattern = u32::from_str_radix(number_no_prefix, radix)
            .expect("Invalid pattern format");
        let words = record
            .iter()
            .skip(1)
            .map(|s| s.trim().to_string())
            .collect();
        Self::new(words, pattern)
    }
    
    fn contains(&self, query: &str) -> bool {
        self.words.iter().any(|word| word.to_lowercase().contains(&query.to_lowercase()))
    }
}

fn import_csv() -> Vec<Atom> {
    // TODO: Let the user choose the file to import
    // Also let the user choose if the CSV has headers
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path("data.csv")
        .expect("Cannot open CSV file");
    reader
        .records()
        .map(|result| {
            let record = result.expect("Error reading CSV record");
            Atom::from_csv_record(&record)
        })
        .collect()
}

fn main() -> iced::Result {
    iced::application("Pixel Editor", App::update, App::view)
        .subscription(App::subscription)
        .run()
}
