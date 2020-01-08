use board::Board;
use std::io;

pub struct Game {
    pub current_turn: char,
}

pub fn new_game() -> Game {
    Game {
        current_turn: 'x',
    }
}

pub fn move_request() -> usize {
    let mut move_request = String::new();
    io::stdin().read_line(&mut move_request).expect("fail");
    return move_request.trim().parse::<usize>().unwrap();
}

impl Game { 
    pub fn is_over(&self, board: &Board) -> bool {
        board.squares[0].value == board.squares[1].value && board.squares[1].value == board.squares[2].value
    }

    pub fn winner(&self) -> char {
        if self.current_turn == 'x' {
            'o'
        } else {
            'x'
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
