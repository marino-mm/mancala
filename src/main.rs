use std::ops::Range;

#[derive(PartialEq, Clone)]
struct Player{
    name:String,
}
struct Game {
    player1: Player,
    player2: Player,
    turn: u8,
    board: [u8; 14]
}

impl Game {

    fn get_player1_score(&self) -> u8 {
        self.board[6]
    }
    fn get_player2_score(&self) -> u8 {
        self.board[13]
    }

    fn standard_game() -> Game {
        Game {
            player1: Player{ name: "player1".to_string()},
            player2: Player{ name: "player2".to_string()},
            turn: 1,
            board: [
                4, 4, 4, 4, 4, 4, 0,4, 4, 4, 4, 4, 4, 0
            ] }
    }

    fn current_players_turn(&self) -> &Player {
        if &self.turn % 2 == 1{
            &self.player1
        }
        else {
            &self.player2
        }
    }

    fn players_pits_limits(&self, player:&Player) -> (u8, u8) {
        if player == &self.player1 {
            (0, 5)
        }else
        {
            (7, 12)
        }
    }

    fn play_pit(&mut self, pit_number: u8) {
        let current_player = &self.current_players_turn().clone();
        let limits = self.players_pits_limits(current_player);
        if limits.0 <= pit_number && pit_number <= limits.1 {
            self.turn += 1;
            let mut gems_number = self.board[pit_number as usize];
            let mut current_pit = pit_number + 1;
            self.board[pit_number as usize] = 0;
            while gems_number > 0 {
                if current_pit == 6 && current_player == &self.player2 {continue;}
                else if current_pit > 13 && current_player == &self.player1 {continue;}
                else {
                    self.board[current_pit as usize] += 1;
                    gems_number -= 1;
                }
                current_pit += 1;
                current_pit = current_pit % 14;
            }
        }
    }

    fn print_board(&mut self) {
        let place_width = 2;
        println!("╔═════════╗");
        println!("║    {:02}   ║", self.board[13]);
        println!("╠════╦════╣");
        for i in 0..6{
            println!("║ {:02} ║ {:02} ║", self.board[i], self.board[12 - i]);
        }
        println!("╠════╩════╣");
        println!("║    {:02}   ║", self.board[6]);
        println!("╚═════════╝");
    }
}

fn main() {
    let mut board = Game::standard_game();
    board.print_board();
    board.play_pit(3);
    board.print_board();
}
