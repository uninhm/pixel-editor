use iced::Task;
use iced::widget::{button, canvas, column, row, text, text_input, Column};
use iced::Length::Fill;
use iced::keyboard;

mod pixel_canvas;
mod custom_widgets;

use pixel_editor::{Grid, Atom, Message};
use crate::pixel_canvas::PixelCanvas;

struct App {
    search_input_string: String,
    atoms: Vec<Atom>,
    grid: Grid<bool>,
    selected_atom: Option<Atom>,
    holding_to_draw: bool,
    mouse_hold_value: bool, // Value to set cells to while mouse is down
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_input_string: String::new(),
            atoms: import_csv(),
            grid: Grid::default(),
            selected_atom: None,
            holding_to_draw: false,
            mouse_hold_value: false,
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
                        .map(|atom| {
                            button(
                                row![
                                    custom_widgets::atom_widget(atom.pattern)
                                        .side_length(20.0),
                                    text(atom.words.join(", "))
                                        .size(20)
                                        .width(Fill)
                                ].spacing(10).padding(5)
                            ).on_press(Message::SelectAtom(atom.clone())).into()
                        })
                )
            } else {
                column![]
            };

        column![
            text_input("Search...", &self.search_input_string)
                .id("search_input")
                .on_input(Message::SearchInputChanged)
                .width(Fill),
            search_results,
            canvas(PixelCanvas::new(
                self.grid.clone(),
                self.selected_atom.clone(),
            ))
                .width(Fill)
                .height(Fill)
        ].padding(10).spacing(10)
    }
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchInputChanged(s) => {
                self.search_input_string = s;
                Task::none()
            },
            Message::FocusSearchInput => {
                text_input::focus("search_input")
            },
            Message::CellClicked(x, y) => {
                // TODO: Hold and drag to draw (keep the first color, do not invert each cell)
                if let Some(atom) = &self.selected_atom {
                    // TODO: Left click to paste only back pixels, right click to paste both
                    // and erase pixels
                    for i in 0..5 {
                        let y = y + i;
                        for j in 0..5 {
                            let x = x + j;
                            self.grid.set(x, y, atom.nth_bit(i * 5 + j));
                        }
                    }
                    self.selected_atom = None;
                } else {
                    self.grid.set(x, y, !self.grid.get(x, y));
                    self.mouse_hold_value = self.grid.get(x, y);
                    self.holding_to_draw = true;
                }
                Task::none()
            },
            Message::SelectAtom(atom) => {
                self.selected_atom = Some(atom);
                Task::none()
            },
            Message::UnselectAtom => {
                self.selected_atom = None;
                Task::none()
            },
            Message::CursorMovedToCell(x, y) => {
                if self.holding_to_draw {
                    self.grid.set(x, y, self.mouse_hold_value);
                }
                Task::none()
            },
            Message::MouseReleased => {
                self.holding_to_draw = false;
                Task::none()
            },
        }
    }
    
    fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| {
            match key.as_ref() {
                keyboard::Key::Character("/") => Some(Message::FocusSearchInput),
                keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::UnselectAtom),
                _ => None,
            }
        })
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
