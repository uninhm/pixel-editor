use iced::advanced::layout::{self, Layout};
use iced::advanced::{renderer, Renderer};
use iced::advanced::widget::{self, Widget};
use iced::border;
use iced::mouse;
use iced::{Color, Element, Length, Rectangle, Size};

pub struct Pattern {
    pattern: u32,
    side_length: f32,
}

impl Pattern {
    pub fn new(pattern: u32) -> Self {
        Self {
            pattern,
            side_length: 30.0,
        }
    }
    
    pub fn side_length(mut self, side_length: f32) -> Self {
        self.side_length = side_length;
        self
    }
}

pub fn pattern(pattern: u32) -> Pattern {
    Pattern::new(pattern)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Pattern
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(
            Size::new(
                self.side_length,
                self.side_length,
            )
        )
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let height = layout.bounds().height;
        let width = layout.bounds().width;
        const PATTERN_LENGTH: usize = 25; // TODO: Make this a setting
        for i in 0 .. PATTERN_LENGTH {
            if self.pattern & (1 << i) != 0 {
                let x = (i % 5) as f32 * (width / 5.0);
                let y = (4 - i / 5) as f32 * (height / 5.0);
                let square = renderer::Quad {
                    bounds: Rectangle {
                        x: x + layout.bounds().x,
                        y: y + layout.bounds().y,
                        width: width / 5.0,
                        height: height / 5.0,
                    },
                    ..renderer::Quad::default()
                };
                renderer.fill_quad(square, Color::BLACK);
            }
        }
    }
}

impl<Message, Theme, Renderer> From<Pattern>
    for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(pattern: Pattern) -> Self {
        Self::new(pattern)
    }
}