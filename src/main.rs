use std::io;

fn main() {
    let mut game_over = false;
    let mut player_turn = 'x';
    let mut board = Board::new();

    while !game_over {
        println!("current game board--");
        board.pretty_print();

        println!("next move is: {}", player_turn);
        println!("where do you want to go?");
        let mut square_move = String::new();
        io::stdin().read_line(&mut square_move).expect("fail");
        let x = square_move.trim().parse::<usize>().unwrap();

        board.squares[x].value = player_turn;

        if board.squares[0].value == board.squares[1].value && board.squares[1].value == board.squares[2].value {
            game_over = true;
        }

        if player_turn == 'x' {
            player_turn = 'o';
        } else {
            player_turn = 'x';
        }
    }

    println!("winner, winner!");
}

struct Square {
    value: char,
}

struct Board {
    squares: [Square; 9],
}

impl Board {
    fn new() -> Board {
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

    fn pretty_print(&self) {
        println!("{}|{}|{}", self.squares[0].value, self.squares[1].value, self.squares[2].value);
        println!("-----");
        println!("{}|{}|{}", self.squares[3].value, self.squares[4].value, self.squares[5].value);
        println!("-----");
        println!("{}|{}|{}", self.squares[6].value, self.squares[7].value, self.squares[8].value);
    }
}

// x|x|x
// -----
// o|o|o
// -----
// x|x|x
