use crossterm::style::{Color, ContentStyle, StyledContent};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Theme{
    pub name: String,
    pub foreground: Color,
    pub background: Color,
    pub highlighted_foreground: Color,
    pub highlighted_background: Color,
}
impl Default for Theme {
    fn default() -> Self {
        // Theme{
        //     name: "default".into(),
        //     foreground: Color::White,
        //     background: Color::Black,
        //     highlighted_foreground: Color::Black,
        //     highlighted_background: Color::White,
        // }
        Theme{
                name: "default".into(),
                foreground: Color::Rgb {r: 255, g: 255, b: 255},
                background: Color::Rgb {r: 0, g: 0, b: 0},
                highlighted_foreground: Color::Rgb {r: 0, g: 0, b: 0},
                highlighted_background: Color::Rgb {r: 255, g: 255, b: 255},
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

    pub fn ema_2() -> Theme {
        Theme{
            name: "Ema_2".to_string(),
            foreground: Color::Rgb { r: 0xff, g: 0x0, b: 0xff },
            background: Color::Rgb { r: 0x75, g: 0x01, b: 0x37 },
            highlighted_foreground: Color::Rgb { r: 0xdb, g: 0x76, b: 0xab },
            highlighted_background: Color::Rgb { r: 0x80, g: 0x0f, b: 0x50 },
        }
    }
}
pub fn color_to_rgb(color: &Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb { r, g, b } => (*r, *g, *b),
        _ => panic!("Please use rgb format of colors")
    }
}


pub struct SelectableText{
    text: String,
    is_highlighted: bool,
    current_theme: Theme
}

impl SelectableText{

    pub fn new(text: String, is_highlighted: bool, theme: Theme) -> SelectableText{
        SelectableText{text, is_highlighted, current_theme: theme}
    }
    pub fn get_styled_content(&self) -> StyledContent<String> {
        match self.is_highlighted{
            true => {
                StyledContent::new(self.current_theme.get_highlight_style(), self.text.clone())
            }
            false => {
                StyledContent::new(self.current_theme.get_content_style(), self.text.clone())
            }
        }
    }
}