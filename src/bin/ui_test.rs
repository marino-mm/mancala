use crossterm::cursor::{Hide, MoveTo, MoveToNextLine};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle};
use crossterm::{execute, queue};
use std::io;
use std::io::{stdout, Write};
use std::time::Duration;


fn move_selected_pit_up(selected_pit_numb: &mut u8) {
    if (1..6).contains(selected_pit_numb) {
        *selected_pit_numb -= 1;
    } else if (7..12).contains(selected_pit_numb) {
        *selected_pit_numb += 1;
    }
}

fn move_selected_pit_left(selected_pit_numb: &mut u8) {
    let n = *selected_pit_numb;

    if (1..6).contains(selected_pit_numb) {
        *selected_pit_numb = 13 - n
    } else if (7..13).contains(selected_pit_numb) {
        *selected_pit_numb = 12 - n
    }
}

fn move_selected_pit_down(selected_pit_numb: &mut u8) {
    if (0..5).contains(selected_pit_numb) {
        *selected_pit_numb += 1;
    } else if (8..13).contains(selected_pit_numb) {
        *selected_pit_numb -= 1;
    }
}

fn move_selected_pit_right(selected_pit_numb: &mut u8) {
    let n = *selected_pit_numb;

    if (0..6).contains(&n) {
        *selected_pit_numb = 12 - n;
    } else if (8..13).contains(&n) {
        // This covers 8->4, 9->3, etc., assuming 13 is the limit
        *selected_pit_numb = 13 - n;
    }
}

fn print_board(selected_pit_numb: &u8){

    let board = [5u8; 14];
    let mut lines = Vec::<String>::with_capacity(20);
    let mut stdout = stdout();
    lines.push("  ╔═════════╗".to_string());
    lines.push(format!("{:02}║    {:02}   ║{:02}",13 ,board[13] ,13 ));
    lines.push("  ╠════╦════╣".to_string());


    let mut left:usize;
    let mut right:usize;

    let mut left_texted_colored:String;
    let mut right_texted_colored:String;

    for i in 0..6 {
        left = i as usize;
        right = (12 - i) as usize;

        if i == *selected_pit_numb{
            left_texted_colored = format!("{:02}", board[left].to_string()).black().on_white().to_string();
            right_texted_colored = board[right].to_string();
        }else if 12 - i == *selected_pit_numb {
            // right_texted_colored = board[right].to_string().black().on_white().to_string();
            right_texted_colored = format!("{:02}", board[right].to_string()).black().on_white().to_string();
            left_texted_colored = board[left].to_string();
        }else {
            right_texted_colored = board[right].to_string();
            left_texted_colored = board[left].to_string();
        }
        lines.push(format!("{:02}║ {:02} ║ {:02} ║{:02}", left, left_texted_colored, right_texted_colored, right));
    }
    lines.push("  ╠════╩════╣".to_string());
    lines.push(format!("{:02}║    {:02}   ║{:02}",6 ,board[6] ,6 ));
    lines.push("  ╚═════════╝".to_string());

    queue!(stdout, MoveTo(0, 1)).unwrap();
    for line in lines {
        queue!(stdout,
                Clear(ClearType::CurrentLine),
                Print(line),
                MoveToNextLine(1)
            ).unwrap();
    }
    stdout.flush().unwrap();
}



fn main() -> io::Result<()> {

    let mut stdout = stdout();
    execute!(stdout,
        EnterAlternateScreen,
        Clear(ClearType::Purge),
        SetTitle("This is a title"),
        MoveTo(0, 0),
        Hide
    )?;
    enable_raw_mode()?;
    let mut selected_pit_numb = 0u8;

    print_board(&selected_pit_numb);
    loop {
        if poll(Duration::from_millis(32))? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                        break;
                    } else if event.code.is_up() {
                        move_selected_pit_up(&mut selected_pit_numb);
                    } else if event.code.is_down() {
                        move_selected_pit_down(&mut selected_pit_numb);
                    } else if event.code.is_left() {
                        move_selected_pit_left(&mut selected_pit_numb);
                    } else if event.code.is_right() {
                        move_selected_pit_right(&mut selected_pit_numb);
                    } else {
                        continue
                    }
                    print_board(&selected_pit_numb);
                    /*
                    queue!(stdout,
                        MoveToColumn(0),
                        Clear(ClearType::CurrentLine),
                        Print(selected_pit_numb.to_string())
                    )?;
                    stdout.flush()?;
                    */
                },
                _ => {}
            }
        } else {
            continue
        }
    }
disable_raw_mode()?;
    execute!(stdout,LeaveAlternateScreen)?;
    Ok(())
}

