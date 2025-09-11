use iced::{Point, Task};
use iced::widget::{button, canvas, column, row, text, text_input, Column};
use iced::Length::Fill;
use iced::{mouse, keyboard, event};
mod custom_widgets;

// TODO: Infinite grid
const GRID_SIZE: usize = 35;
#[derive(Clone)]
struct Grid<T> {
    grid: Vec<Vec<T>>
}

impl<T> Default for Grid<T> where T: Default + Clone {
    fn default() -> Self {
        Self {
            grid: vec![vec![T::default(); GRID_SIZE]; GRID_SIZE]
        }
    }
}

impl<T> Grid<T> where T: Default + Copy {
    fn get(&self, x: usize, y: usize) -> T {
        if x < GRID_SIZE && y < GRID_SIZE {
            self.grid[y][x]
        } else {
            T::default()
        }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        if x < GRID_SIZE && y < GRID_SIZE {
            self.grid[y][x] = val;
        } else {
            println!("Warning: Tried to set out-of-bounds grid cell ({}, {})", x, y);
        }
    }
}

#[derive(Default)]
struct PixelCanvas {
    grid: Grid<bool>,
    selected_atom: Option<Atom>,
}

#[derive(Default)]
struct CanvasState {
    mouse_pos: Point,
}

impl canvas::Program<Message> for PixelCanvas {
    type State = CanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        let vert_cell_count = 35.0;
        let cell_size = bounds.height/vert_cell_count;
        match event {
            canvas::Event::Mouse(e) => {
                match e {
                    mouse::Event::CursorMoved{position} => {
                        state.mouse_pos = position;
                        (event::Status::Captured, None)
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if !bounds.contains(state.mouse_pos) {
                            return (event::Status::Ignored, None);
                        }
                        let x: usize = ((state.mouse_pos.x - bounds.x) / cell_size) as usize;
                        let y: usize = ((state.mouse_pos.y - bounds.y) / cell_size) as usize;
                        (event::Status::Captured, Some(Message::CellClicked(x, y)))
                    }
                    _ => (event::Status::Ignored, None),
                }
            },
            _ => {
                (event::Status::Ignored, None)
            }
        }
    }
    
    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let vert_cell_count = 35.0;
        let cell_size = bounds.height/vert_cell_count;
        let horz_cell_count = bounds.width/cell_size;
        let stroke = canvas::Stroke::default()
            .with_width(0.7);
        
        // Draw grid lines
        for i in 0..=vert_cell_count as i32 {
            let y = (i as f32 * cell_size).floor() + 0.5; // floor+0.5 so the lines are on the middle of a pixel
            let line = canvas::Path::line(
                iced::Point::new(0.0, y),
                iced::Point::new(bounds.width, y)
            );
            frame.stroke(&line, stroke);
        }
        for i in 0..=horz_cell_count as i32 {
            let x = (i as f32 * cell_size).floor() + 0.5; // floor+0.5 so the lines are on the middle of a pixel
            let line = canvas::Path::line(
                iced::Point::new(x, 0.0),
                iced::Point::new(x, bounds.height)
            );
            frame.stroke(&line, stroke);
        }

        // Draw the black squares
        for i in 0..=vert_cell_count as i64 {
            let y = (i as f32 * cell_size).floor() + 0.5;
            for j in 0..=vert_cell_count as i64 {
                let x = (j as f32 * cell_size).floor() + 0.5;
                let rect = canvas::Path::rectangle(
                    Point::new(x, y),
                    iced::Size::new(cell_size, cell_size),
                );
                if self.grid.get(j as usize, i as usize) {
                    frame.fill(
                        &rect,
                        iced::Color::BLACK
                    );
                }
            }
        }
        
        if bounds.contains(state.mouse_pos) && let Some(atom) = &self.selected_atom {
            let mouse_relative_x = state.mouse_pos.x - bounds.x;
            let mouse_relative_y = state.mouse_pos.y - bounds.y;
            let start_x = ((mouse_relative_x / cell_size).floor() * cell_size).max(0.0);
            let start_y = ((mouse_relative_y / cell_size).floor() * cell_size).max(0.0);
            for i in 0..5 {
                for j in 0..5 {
                    let x = start_x + (j as f32 * cell_size);
                    let y = start_y + (i as f32 * cell_size);
                    let rect = canvas::Path::rectangle(
                        Point::new(x, y),
                        iced::Size::new(cell_size, cell_size),
                    );
                    if atom.nth_bit(i * 5 + j) {
                        frame.fill(
                            &rect,
                            iced::Color::from_rgb(0.0, 0.0,1.0) 
                        );
                    }
                }
            }
        }

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    FocusSearchInput,
    CellClicked(usize, usize),
    SelectAtom(Atom),
    UnselectAtom,
}

struct App {
    search_input_string: String,
    atoms: Vec<Atom>,
    grid: Grid<bool>,
    selected_atom: Option<Atom>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_input_string: String::new(),
            atoms: import_csv(),
            grid: Grid::default(),
            selected_atom: None,
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
                                    custom_widgets::pattern(atom.pattern)
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
            canvas(PixelCanvas {
                grid: self.grid.clone(),
                selected_atom: self.selected_atom.clone(),
                ..Default::default()
            })
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
                if let Some(atom) = &self.selected_atom {
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
            }
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

#[derive(Clone, Debug)]
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
    
    fn nth_bit(&self, n: usize) -> bool {
        if n >= 25 {
            panic!("Bit index out of range");
        }
        (self.pattern >> (24 - n)) & 1 != 0
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
