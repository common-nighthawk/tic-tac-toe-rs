use board::Board;
use std::io;

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

pub fn move_request() -> usize {
    let mut move_request = String::new();
    io::stdin().read_line(&mut move_request).expect("fail");
    return move_request.trim().parse::<usize>().unwrap();
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

    pub fn winner(&self) -> char {
        match self.current_turn {
            X => O,
            _ => X,
        }
    }

    pub fn change_turn(&mut self) {
        if self.current_turn == 'x' {
            self.current_turn = 'o';
        } else {
            self.current_turn = 'x';
        }
    }
}
