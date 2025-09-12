use iced::Point;
use iced::widget::canvas;
use iced::{mouse, event};

use pixel_editor::{Message, Grid, GridIndex, Atom};

#[derive(Default)]
pub struct PixelCanvas {
    grid: Grid<bool>,
    selected_atom: Option<Atom>,
    cell_size: f32,
}

impl PixelCanvas {
    pub fn new(
        grid: Grid<bool>,
        selected_atom: Option<Atom>,
        cell_size: f32,
    ) -> Self {
        Self { grid, selected_atom, cell_size }
    }
}

#[derive(Default)]
pub struct CanvasState {
    mouse_pos: Point,
    top_left: Point,
    middle_button_start: Option<Point>,
    middle_button_top_left_start: Option<Point>,
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
        let x: GridIndex = ((state.mouse_pos.x - bounds.x + state.top_left.x) / self.cell_size).floor() as GridIndex;
        let y: GridIndex = ((state.mouse_pos.y - bounds.y + state.top_left.y) / self.cell_size).floor() as GridIndex;
        match event {
            canvas::Event::Mouse(e) => {
                match e {
                    mouse::Event::CursorMoved{position} => {
                        state.mouse_pos = position;
                        if let Some(start) = state.middle_button_start {
                            let middle_button_start = state.middle_button_top_left_start
                                .expect("Middle button start position should be set if middle button is held"); 
                            state.top_left.x = middle_button_start.x - (state.mouse_pos.x - start.x);
                            state.top_left.y = middle_button_start.y - (state.mouse_pos.y - start.y);
                        }
                        (event::Status::Captured, Some(Message::CursorMovedToCell(x, y)))
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if !bounds.contains(state.mouse_pos) {
                            return (event::Status::Ignored, None);
                        }
                        (event::Status::Captured, Some(Message::CellClicked(x, y)))
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Middle) => {
                        if !bounds.contains(state.mouse_pos) {
                            return (event::Status::Ignored, None);
                        }
                        state.middle_button_start = Some(state.mouse_pos);
                        state.middle_button_top_left_start = Some(state.top_left);
                        (event::Status::Captured, None)
                    },
                    mouse::Event::WheelScrolled { delta } => {
                        match delta {
                            mouse::ScrollDelta::Lines { y, .. } => {
                                if y > 0.0 {
                                    (event::Status::Captured, Some(Message::ZoomIn))
                                } else if y < 0.0 {
                                    (event::Status::Captured, Some(Message::ZoomOut))
                                } else {
                                    (event::Status::Ignored, None)
                                }
                            },
                            mouse::ScrollDelta::Pixels { y, .. } => {
                                if y > 0.0 {
                                    (event::Status::Captured, Some(Message::ZoomIn))
                                } else if y < 0.0 {
                                    (event::Status::Captured, Some(Message::ZoomOut))
                                } else {
                                    (event::Status::Ignored, None)
                                }
                            },
                        }
                    },
                    mouse::Event::ButtonReleased(mouse::Button::Middle) => {
                        state.middle_button_start = None;
                        (event::Status::Captured, None)
                    },
                    mouse::Event::ButtonReleased(mouse::Button::Left) =>
                        (event::Status::Captured, Some(Message::MouseReleased)),
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
        let vert_cell_count = bounds.height/self.cell_size + 1.0;
        let horz_cell_count = bounds.width/self.cell_size + 1.0;
        let stroke = canvas::Stroke::default()
            .with_width(0.7);
        let mod_y = if state.top_left.y < 0.0 {
            (state.top_left.y % self.cell_size + self.cell_size) % self.cell_size
        } else {
            state.top_left.y % self.cell_size
        };
        let mod_x = if state.top_left.x < 0.0 {
            (state.top_left.x % self.cell_size + self.cell_size) % self.cell_size
        } else {
            state.top_left.x % self.cell_size
        };
        
        // Draw grid lines
        for i in 0..=vert_cell_count as i32 {
            let y = (i as f32 * self.cell_size) - state.top_left.y % self.cell_size;
            let line = canvas::Path::line(
                iced::Point::new(0.0, y),
                iced::Point::new(bounds.width, y)
            );
            frame.stroke(&line, stroke);
        }
        for i in 0..=horz_cell_count as i32 {
            let x = (i as f32 * self.cell_size).floor() - state.top_left.x % self.cell_size;
            let line = canvas::Path::line(
                iced::Point::new(x, 0.0),
                iced::Point::new(x, bounds.height)
            );
            frame.stroke(&line, stroke);
        }

        // Draw the black squares
        // TODO: Add color settings
        let start_x = (state.top_left.x / self.cell_size).floor() as i64;
        let start_y = (state.top_left.y / self.cell_size).floor() as i64;
        for i in 0..=vert_cell_count as i64 {
            let y = i as f32 * self.cell_size - mod_y;
            for j in 0..=horz_cell_count as i64 {
                let x = j as f32 * self.cell_size - mod_x;
                let rect = canvas::Path::rectangle(
                    Point::new(x, y),
                    iced::Size::new(self.cell_size, self.cell_size),
                );
                if self.grid.get(start_x + j , start_y + i) {
                    frame.fill(
                        &rect,
                        iced::Color::BLACK
                    );
                }
            }
        }
        
        if bounds.contains(state.mouse_pos) && let Some(atom) = &self.selected_atom {
            let mouse_relative_x = state.mouse_pos.x - bounds.x + mod_x;
            let mouse_relative_y = state.mouse_pos.y - bounds.y + mod_y;
            let start_x = (mouse_relative_x / self.cell_size).floor() * self.cell_size - mod_x;
            let start_y = (mouse_relative_y / self.cell_size).floor() * self.cell_size - mod_y;
            for i in 0..5 {
                for j in 0..5 {
                    let x = start_x + (j as f32 * self.cell_size);
                    let y = start_y + (i as f32 * self.cell_size);
                    let rect = canvas::Path::rectangle(
                        Point::new(x, y),
                        iced::Size::new(self.cell_size, self.cell_size),
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
