use crossterm::style::{Color, ContentStyle};

pub struct Theme{
    pub foreground: Color,
    pub background: Color,
    pub highlighted_foreground: Color,
    pub highlighted_background: Color,
}
impl Default for Theme {
    fn default() -> Self {
        Theme{
            foreground: Color::White,
            background: Color::Black,
            highlighted_foreground: Color::Black,
            highlighted_background: Color::White,
        }
    }
}

impl Theme {
    pub fn get_content_style(&self) -> ContentStyle {
        ContentStyle{
            foreground_color: Some(self.foreground),
            background_color: Some(self.background),
            underline_color: None,
            attributes: Default::default(),
        }
    }
    pub fn get_highlight_style(&self) -> ContentStyle {
        ContentStyle{
            foreground_color: Some(self.highlighted_foreground),
            background_color: Some(self.highlighted_background),
            underline_color: None,
            attributes: Default::default(),
        }
    }

    pub fn ema() -> Theme {
        Theme{
            foreground: Color::Cyan,
            background: Color::Rgb { r: 0xd7, g: 0xba, b: 89 },
            highlighted_foreground: Color::Red,
            highlighted_background: Color::Blue,
        }
    }
}