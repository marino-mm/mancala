use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdin, stdout, BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

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
}

impl Game {
    fn get_player_score_pit_position(&self, player_id: u8) -> u8 {
        if self.player1.id == player_id { 6 } else { 13 }
    }

    fn get_pit_across(&self, pit_numb: u8) -> usize {
        (12 - pit_numb) as usize
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
            status: "running".to_string(),
            is_free_turn: false,
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

    fn no_valid_moves_for_current_player(&self) -> bool {
        if self.current_player_id == self.player1.id {
            for i in 0..6 {
                if !self.board[i] != 0 {
                    return false;
                }
            }
        } else {
            for i in 7..13 {
                if !self.board[i] != 0 {
                    return false;
                }
            }
        }
        true
    }

    fn end_game(&mut self) {
        for i in 0..6 {
            self.board[self.get_player_score_pit_position(1) as usize] += self.board[i];
            self.board[i] = 0;
        }
        for i in 7..13 {
            self.board[self.get_player_score_pit_position(2) as usize] += self.board[i];
            self.board[i] = 0;
        }
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
            if self.board[current_pit as usize] == 1
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
            if self.is_free_move(current_pit) {
                self.is_free_turn = true;
            }
            else {
                self.is_free_turn = false;
            }
    }

    fn read_move(&self, input:&mut String) -> u8{
        stdin().read_line(input).unwrap();
        loop {
            match input.trim().parse::<u8>() {
                Ok(num) => {
                    input.clear();
                    return num;
                }
                Err(_) => {
                    input.clear();
                    println!("Invalid input");
                }
            }
        }
    }

    fn start_game_loop(&mut self) {
        let mut input = String::new();
        let mut move_number = 0;

        self.print_board();
        while self.status != "end" {

            if self.no_valid_moves_for_current_player(){
                self.end_game();
                return;
            }

            move_number = self.read_move(&mut input);
            if self.is_players_pit(self.current_player_id, move_number as usize)
            && (self.board[move_number as usize] != 0){
                self.play_move(move_number);
            }
            else {
                println!("Invalid move number");
            }

            if self.is_free_turn {
                println!("It is your free move");
                self.print_board();
            } else {
                self.switch_current_player();
                self.print_board();
            }

        }
    }

    fn print_board(&self){
        let mut lines = Vec::<String>::with_capacity(20);

        lines.push("╔═════════╗".to_string());
        lines.push(format!("║    {:02}   ║",self.board[self.get_player_score_pit_position(2) as usize]));
        lines.push("╠════╦════╣".to_string());
        for i in 0..6 {
            lines.push(format!("║ {:02} ║ {:02} ║",self.board[i],self.board[12 - i]));
        }
        lines.push("╠════╩════╣".to_string());
        lines.push(format!("║    {:02}   ║",self.board[self.get_player_score_pit_position(1) as usize]));
        lines.push("╚═════════╝".to_string());

        stdout().queue(cursor::MoveTo(0, 1)).unwrap();
        for line in lines {
            stdout().queue(Clear(ClearType::CurrentLine)).unwrap();
            stdout().queue(Print(line)).unwrap();
            stdout().queue(cursor::MoveToNextLine(1)).unwrap();
        }
        stdout().flush().unwrap();
    }

    fn print_header(&self, text: String) {
        stdout().queue(cursor::MoveTo(0,0));
        stdout().queue(Clear(ClearType::CurrentLine));
        stdout().queue(Print(text)).unwrap();
        stdout().flush().unwrap();
    }

    fn print_footer(&self, text: String) {
        stdout().queue(cursor::MoveTo(0,13));
        stdout().queue(Clear(ClearType::CurrentLine));
        stdout().queue(Print(text)).unwrap();
        stdout().flush().unwrap();
    }
}

fn main() {
    let mut game = Game::standard_game();
    // game.start_game_loop();
    stdout().queue(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    stdout().queue(cursor::Hide).unwrap();

    game.print_header(String::from("This is header"));
    game.print_board();
    sleep(Duration::from_secs(5));
    game.play_move(0);
    game.print_board();
    sleep(Duration::from_secs(5));
    game.print_footer("This is footer".to_string());
    sleep(Duration::from_secs(5));

    disable_raw_mode().unwrap();
    stdout().queue(LeaveAlternateScreen).unwrap();
}
