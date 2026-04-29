use std::fmt::{Display, Formatter};
use crossterm::style::{Color, ContentStyle};

pub struct Theme{
    pub name: String,
    pub foreground: Color,
    pub background: Color,
    pub highlighted_foreground: Color,
    pub highlighted_background: Color,
}
impl Default for Theme {
    fn default() -> Self {
        Theme{
            name: "default".into(),
            foreground: Color::White,
            background: Color::Black,
            highlighted_foreground: Color::Black,
            highlighted_background: Color::White,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(&*self.name)
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
            name: "ema".into(),
            foreground: Color::Rgb { r: 0x9a, g: 0x78, b: 0x4f },
            background: Color::Rgb { r: 0x43, g: 0x26, b: 0x16 },
            highlighted_foreground: Color::Rgb { r: 0x43, g: 0x26, b: 0x16 },
            highlighted_background: Color::Rgb { r: 0x9a, g: 0x78, b: 0x4f },
        }
    }
}