use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, queue, QueueableCommand};
use cursor::MoveTo;
use std::io;
use std::io::{stdout, Write};
use std::process::exit;
use std::time::Duration;
use ClearType::CurrentLine;
use rand::RngExt;

#[derive(PartialEq, Clone)]
struct Player {
    id: u8,
    name: String,
}
struct Game {
    player1: Player,
    player2: Player,
    turn: u8,
    current_player_id: u8,
    board: [u8; 14],
    status: String,
    is_free_turn: bool,
    selected_pit_number: u8,
}

impl Game {
    fn get_player_score_pit_position(&self, player_id: u8) -> u8 {
        if self.player1.id == player_id { 6 } else { 13 }
    }

    fn get_pit_across(&self, pit_numb: u8) -> usize {
        match pit_numb {
            n if n == 6 || n == 13=> {n as usize}
            _ => (12u8 - pit_numb) as usize
        }
    }

    fn standard_game() -> Game {
        stdout().queue(Clear(ClearType::All)).unwrap();
        Game {
            player1: Player {
                id: 1,
                name: "player1".to_string(),
            },
            player2: Player {
                id: 2,
                name: "player2".to_string(),
            },
            turn: 1,
            current_player_id: 1,
            board: [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0],
            // board: [0, 0, 0, 0, 0, 1, 0, 4, 4, 4, 4, 4, 4, 0],
            status: "running".to_string(),
            is_free_turn: false,
            selected_pit_number: 0,
        }
    }

    fn random_game() -> Game {
        let mut board = [0; 14];
        let mut rng = rand::rng();
        for n in 0..6{
            let x = rng.random_range(1..=7);
            board[n] = x;
            board[n+7] = x;
        }
        Game {
            player1: Player {
                id: 1,
                name: "player1".to_string(),
            },
            player2: Player {
                id: 2,
                name: "player2".to_string(),
            },
            turn: 1,
            current_player_id: 1,
            board,
            status: "running".to_string(),
            is_free_turn: false,
            selected_pit_number: 0,
        }
    }

    fn move_selected_pit_up(&mut self) {
        if (1..6).contains(&self.selected_pit_number) {
            self.selected_pit_number -= 1;
        } else if (7..12).contains(&self.selected_pit_number) {
            self.selected_pit_number += 1;
        }
    }

    fn move_selected_pit_left(&mut self) {
        let n = self.selected_pit_number;

        if (1..6).contains(&self.selected_pit_number) {
            self.selected_pit_number = 13 - n
        } else if (7..13).contains(&self.selected_pit_number) {
            self.selected_pit_number = 12 - n
        }
    }

    fn move_selected_pit_down(&mut self) {
        if (0..5).contains(&self.selected_pit_number) {
            self.selected_pit_number += 1;
        } else if (8..13).contains(&self.selected_pit_number) {
            self.selected_pit_number -= 1;
        }
    }

    fn move_selected_pit_right(&mut self) {
        let n = self.selected_pit_number;

        if (0..6).contains(&n) {
            self.selected_pit_number = 12 - n;
        } else if (8..13).contains(&n) {
            self.selected_pit_number = 13 - n;
        }
    }

    fn is_players_pit(&self, player_id: u8, pit_number: usize) -> bool {
        (self.player1.id == player_id && pit_number < 6)
            || (self.player2.id == player_id && (7..13).contains(&pit_number))
    }

    fn switch_current_player(&mut self) {
        if self.player1.id == self.current_player_id {
            self.current_player_id = self.player2.id;
        } else {
            self.current_player_id = self.player1.id;
        }
    }

    fn is_free_move(&self, last_pit: u8) -> bool {
        (self.current_player_id == self.player1.id
            && self.get_player_score_pit_position(self.player1.id) == last_pit)
            || (self.current_player_id == self.player2.id
                && self.get_player_score_pit_position(self.player2.id) == last_pit)
    }

    fn no_valid_moves_for_eather_player(&self) -> bool {
        let mut player1_valid_pits = 0;
        let mut player2_valid_pits = 0;
        for n in 0..6 {
            if self.board[n] != 0 {
                player1_valid_pits += 1;
            }
            if self.board[n + 7] != 0 {
                player2_valid_pits += 1;
            }
        }
        if player1_valid_pits == 0 || player2_valid_pits == 0 {
             return true;
        }
        false
    }

    fn end_game(&mut self) {
        // self.print_footer_debug("Enterde end game".to_string());
        for i in 0..6 {
            self.board[self.get_player_score_pit_position(1) as usize] += self.board[i];
            self.board[i] = 0;
        }
        for i in 7..13 {
            self.board[self.get_player_score_pit_position(2) as usize] += self.board[i];
            self.board[i] = 0;
        }

        let player1_point = self.board[self.get_player_score_pit_position(1) as usize];
        let player2_point = self.board[self.get_player_score_pit_position(2) as usize];
        let mut end_string;
        if player1_point > player2_point {
            end_string = format!("The winner is {}", self.player1.name.clone());
        } else if player2_point > player1_point {
            end_string = format!("The winner is {}", self.player2.name.clone());
        } else {
            end_string = "It is a draw".to_string();
        }
        end_string = end_string + "\nPress enter to exit.";
        self.print_footer(end_string);
        self.print_board();
        self.status = "end".to_string();
    }

    fn play_move(&mut self, pit_number: u8){
            self.turn += 1;
            let mut gems_number = self.board[pit_number as usize];
            let mut current_pit = pit_number;
            self.board[pit_number as usize] = 0;
            while gems_number > 0 {
                current_pit += 1;
                current_pit = current_pit % 14;
                if current_pit == 6 && self.current_player_id == self.player2.id {
                    continue;
                } else if current_pit == 13 && self.current_player_id == self.player1.id {
                    continue;
                } else {
                    self.board[current_pit as usize] += 1;
                    gems_number -= 1;
                }
            }
            let pit_across_numb = self.get_pit_across(current_pit);
            if self.is_free_move(current_pit) {
                self.is_free_turn = true;
            } else if self.board[current_pit as usize] == 1
                && self.board[pit_across_numb] != 0
                && self.is_players_pit(self.current_player_id, pit_number as usize)
            {
                let current_player = self.current_player_id;
                let player_pit_number = self.get_player_score_pit_position(current_player);

                self.board[player_pit_number as usize] += 1;
                self.board[current_pit as usize] = 0;

                self.board[player_pit_number as usize] += self.board[pit_across_numb];
                self.board[pit_across_numb] = 0;
            }
            else {
                self.is_free_turn = false;
            }
    }

    fn start_game_loop(&mut self) -> Result<(), io::Error> {
        let mut current_player_name:String;

        current_player_name = match self.current_player_id {
            id if id == self.player1.id => self.player1.name.clone(),
            _ => self.player2.name.clone()
        };
        self.print_header(
            format!("Current player name: {}", current_player_name)
        );
        self.print_board();
        while self.status != "end" {
                if poll(Duration::from_millis(100))? {
                match read()? {
                    Event::Key(event) => {
                        if event.kind.is_press() {
                            if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                                exit_game()
                            } else if event.code.is_enter() {
                                let move_number = self.selected_pit_number.clone();
                                if self.is_players_pit(self.current_player_id, move_number as usize) && (self.board[move_number as usize] != 0) {
                                    self.play_move(move_number);

                                    if self.no_valid_moves_for_eather_player(){
                                            self.end_game();
                                            return Ok(());
                                        }
                                    if self.is_free_turn {
                                        self.print_footer("It is your free move".to_string());
                                        self.print_board();
                                    } else {
                                        self.switch_current_player();

                                        current_player_name = match self.current_player_id {
                                            id if id == self.player1.id => self.player1.name.clone(),
                                            _ => self.player2.name.clone()
                                        };
                                        self.print_header(
                                            format!("Current player name: {}", current_player_name)
                                        );
                                        self.print_board();
                                    }
                                } else {
                                    self.print_footer("Invalid move number".to_string());
                                }
                            } else if event.code.is_up() {
                                self.move_selected_pit_up();
                            } else if event.code.is_down() {
                                self.move_selected_pit_down();
                            } else if event.code.is_left() {
                                self.move_selected_pit_left();
                            } else if event.code.is_right() {
                                self.move_selected_pit_right();
                            } else {
                                continue
                            }
                        }
                        self.print_board()
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn print_board(&self){
        let mut lines = Vec::<String>::with_capacity(20);
        let mut stdout = stdout();

        lines.push("  ╔═════════╗".to_string());
        lines.push(format!("{:02}║    {:02}   ║{:02}", self.get_player_score_pit_position(2),self.board[self.get_player_score_pit_position(2) as usize], self.get_player_score_pit_position(2)));
        lines.push("  ╠════╦════╣".to_string());

        let mut left:usize;
        let mut right:usize;

        let mut left_texted_colored:String;
        let mut right_texted_colored:String;

        for i in 0..6 {
            left = i;
            right = 12 - i;

            // lines.push(format!("{:02}║ {:02} ║ {:02} ║{:02}", i, self.board[i], self.board[12 - i], 12 - i));

            if i == self.selected_pit_number as usize {
                left_texted_colored = format!("{:02}", self.board[left].to_string()).black().on_white().to_string();
                right_texted_colored = self.board[right].to_string();
            }else if 12 - i == self.selected_pit_number as usize {
                // right_texted_colored = board[right].to_string().black().on_white().to_string();
                right_texted_colored = format!("{:02}", self.board[right].to_string()).black().on_white().to_string();
                left_texted_colored = self.board[left].to_string();
            }else {
                right_texted_colored = self.board[right].to_string();
                left_texted_colored = self.board[left].to_string();
            }
            lines.push(format!("{:02}║ {:02} ║ {:02} ║{:02}", left, left_texted_colored, right_texted_colored, right));
        }
        lines.push("  ╠════╩════╣".to_string());
        lines.push(format!("{:02}║    {:02}   ║{:02}", self.get_player_score_pit_position(1),self.board[self.get_player_score_pit_position(1) as usize], self.get_player_score_pit_position(1)));
        lines.push("  ╚═════════╝".to_string());

        queue!(stdout, MoveTo(0, 1)).unwrap();
        for line in lines {
            queue!(stdout,
                Clear(CurrentLine),
                Print(line),
                cursor::MoveToNextLine(1)
            ).unwrap();
        }
        stdout.flush().unwrap();
    }

    fn print_header(&self, text: String) {
        let mut stdout = stdout();
        queue!(stdout,
            MoveTo(0,0),
            Clear(CurrentLine),
            Print(text)
        ).unwrap();
        stdout.flush().unwrap();
    }

    fn print_footer(&self, text: String) {
        let mut stdout = stdout();
        queue!(stdout,
            MoveTo(0,13),
            Clear(CurrentLine),
            Print(text)
        ).unwrap();
        stdout.flush().unwrap();
    }

    fn print_footer_debug(&self, text: String) {
        let mut stdout = stdout();
        queue!(stdout,
            MoveTo(0,15),
            Clear(CurrentLine),
            Print(text)
        ).unwrap();
        stdout.flush().unwrap();
    }
}

fn setup_game_screen(){
    let mut stdout = stdout();
    queue!(stdout,
        EnterAlternateScreen,
        cursor::Hide
    ).unwrap();
    enable_raw_mode().unwrap();
    stdout.flush().unwrap();
}

fn exit_game(){
    let mut stdout = stdout();

    loop {
        if poll(Duration::from_millis(100)).unwrap(){
            match read().unwrap() {
                Event::Key(event) => {
                    if  event.is_press(){
                        if event.code.is_enter(){
                            break;
                        }
                    }
                }
                _ => {continue}
            }
        }
    }

    disable_raw_mode().unwrap();
    queue!(stdout,
        cursor::Show,
        LeaveAlternateScreen
    ).unwrap();
    stdout.flush().unwrap();
    exit(0);
}

fn main() {
    setup_game_screen();
    // let mut game = Game::standard_game();
    let mut game = Game::random_game();
    game.start_game_loop().unwrap();

    exit_game();
}
