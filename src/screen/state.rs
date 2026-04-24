use crate::app::App;
use crossterm::event::Event;

pub trait State {
    fn render(&mut self, app: &App);
    fn handel_input(self: Box<Self>, event: Event, app: &mut App) -> Box<dyn State>;
    fn render_next(&self, app: &mut App) -> bool;
}