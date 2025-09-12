use iced::Point;
use iced::widget::canvas;
use iced::{mouse, event};

use pixel_editor::{Message, Grid, GridIndex, Atom};

#[derive(Default)]
pub struct PixelCanvas {
    grid: Grid<bool>,
    selected_atom: Option<Atom>,
}

impl PixelCanvas {
    pub fn new(grid: Grid<bool>, selected_atom: Option<Atom>) -> Self {
        Self { grid, selected_atom }
    }
}

#[derive(Default)]
pub struct CanvasState {
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
        let cell_size = (bounds.height/vert_cell_count).floor();
        match event {
            canvas::Event::Mouse(e) => {
                match e {
                    mouse::Event::CursorMoved{position} => {
                        state.mouse_pos = position;
                        let x: GridIndex = ((state.mouse_pos.x - bounds.x) / cell_size) as GridIndex;
                        let y: GridIndex = ((state.mouse_pos.y - bounds.y) / cell_size) as GridIndex;
                        (event::Status::Captured, Some(Message::CursorMovedToCell(x, y)))
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if !bounds.contains(state.mouse_pos) {
                            return (event::Status::Ignored, None);
                        }
                        let x: GridIndex = ((state.mouse_pos.x - bounds.x) / cell_size) as GridIndex;
                        let y: GridIndex = ((state.mouse_pos.y - bounds.y) / cell_size) as GridIndex;
                        (event::Status::Captured, Some(Message::CellClicked(x, y)))
                    }
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
        let vert_cell_count = 35.0;
        let cell_size = (bounds.height/vert_cell_count).floor();
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
        // TODO: Add color settings
        for i in 0..=vert_cell_count as i64 {
            let y = i as f32 * cell_size + 0.5;
            for j in 0..=horz_cell_count as i64 {
                let x = j as f32 * cell_size + 0.5;
                let rect = canvas::Path::rectangle(
                    Point::new(x, y),
                    iced::Size::new(cell_size, cell_size),
                );
                if self.grid.get(j as GridIndex, i as GridIndex) {
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
            let start_x = ((mouse_relative_x / cell_size).floor() * cell_size).max(0.0) + 0.5;
            let start_y = ((mouse_relative_y / cell_size).floor() * cell_size).max(0.0) + 0.5;
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
