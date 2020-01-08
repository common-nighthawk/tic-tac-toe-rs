use board::Board;
use std::io;

// should these be an enum?
const X: char = 'x';
const O: char = 'o';

pub struct Game {
    pub current_turn: char,
}

pub fn new_game() -> Game {
    Game {
        current_turn: X,
    }
}

pub fn get_move_index() -> usize {
    loop {
        println!("where do you want to go?");

        let mut move_request = String::new();
        match io::stdin().read_line(&mut move_request) {
            Ok(_) => (),
            Err(error) => {
                println!("Could not read value. Please try again.");
                println!("{:?}", error);
                continue;
            }
        }

        let parsed_value = move_request.trim().parse::<usize>();
        let value = match parsed_value {
            Ok(v) => v,
            Err(error) => {
                println!("Could not convert value to positive integer. Please try again.");
                println!("{:?}", error);
                continue;
            }
        };

        if value > 8 {
            println!("Integer must be a number 0-8. Please try again.");
            continue;
        }

        return value;
    }
}

impl Game { 
    pub fn is_over(&self, board: &Board) -> bool {
        // horizontal win
        (board.squares[0].value == board.squares[1].value && board.squares[1].value == board.squares[2].value) ||
        (board.squares[3].value == board.squares[4].value && board.squares[4].value == board.squares[5].value) ||
        (board.squares[6].value == board.squares[7].value && board.squares[7].value == board.squares[8].value) ||
        // vertical win
        (board.squares[0].value == board.squares[3].value && board.squares[3].value == board.squares[6].value) ||
        (board.squares[1].value == board.squares[4].value && board.squares[4].value == board.squares[7].value) ||
        (board.squares[2].value == board.squares[5].value && board.squares[5].value == board.squares[8].value) ||
        // diagonal win
        (board.squares[0].value == board.squares[4].value && board.squares[4].value == board.squares[8].value) ||
        (board.squares[2].value == board.squares[4].value && board.squares[4].value == board.squares[6].value)
    }

    //enum will help?
    pub fn change_turn(&mut self) {
        if self.current_turn == X {
            self.current_turn = O;
        } else {
            self.current_turn = X;
        }
    }
}
