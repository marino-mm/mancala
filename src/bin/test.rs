use crossterm::event::{poll, read};
use mancala::app::App;
use mancala::screen::{MainMenu, State};
use std::time::Duration;
use std::{io, thread};


fn start_game() -> io::Result<()>{
    let mut app = App::new();
    let mut state:Box<dyn State> = Box::new(MainMenu{});

    while app.running{
        if poll(Duration::from_millis(0))? {
            state = state.handel_input(read()?,&mut app)
        }
        state.render(&app);
        thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}


fn main() -> io::Result<()> {
    start_game()
}