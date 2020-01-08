pub struct Square {
    pub value: char,
}

pub struct Board {
    pub squares: [Square; 9],
}

impl Board {
    pub fn pretty_print(&self) {
        println!("{}|{}|{}", self.squares[0].value, self.squares[1].value, self.squares[2].value);
        println!("-----");
        println!("{}|{}|{}", self.squares[3].value, self.squares[4].value, self.squares[5].value);
        println!("-----");
        println!("{}|{}|{}", self.squares[6].value, self.squares[7].value, self.squares[8].value);
    }

    pub fn update_square(&mut self, move_index: usize, current_turn: char) {
        self.squares[move_index].value = current_turn
    }
}

pub fn new_board() -> Board {
    Board {
        squares: [
            Square { value: '0' },
            Square { value: '1' },
            Square { value: '2' },
            Square { value: '3' },
            Square { value: '4' },
            Square { value: '5' },
            Square { value: '6' },
            Square { value: '7' },
            Square { value: '8' },
        ]
    }
}

