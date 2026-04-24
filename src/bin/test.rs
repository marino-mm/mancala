use crossterm::event::{poll, read};
use mancala::app::App;
use std::time::{Duration, Instant};
use std::{io, thread};
use mancala::screen::main_menu::MainMenu;
use mancala::screen::state::State;

fn start_game() -> io::Result<()>{
    let mut app = App::new();
    let mut state:Box<dyn State> = Box::new(MainMenu::new());

    let target_fps = 60;
    let target_duration_micros = 1000000/target_fps;
    let mut start_time;

    state.render(&app);
    while app.running{
        start_time = Instant::now();
        if poll(Duration::from_millis(0))? {
            state = state.handel_input(read()?,&mut app);
        }
        if state.render_next(&mut app){
            state.render(&app);
        }
        thread::sleep(Duration::from_micros(target_duration_micros - start_time.elapsed().as_micros() as u64));
    }

    Ok(())
}


fn main() -> io::Result<()> {
    start_game()
}