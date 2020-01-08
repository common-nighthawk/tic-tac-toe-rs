pub mod board;
pub mod game;

fn main() {
    let mut my_game = game::new_game();
    let mut my_board = board::new_board();

    loop {
        println!("current game board--");
        my_board.pretty_print();

        println!("next move is: {}", my_game.current_turn);
        let move_index = game::get_move_index();

        my_board.update_square(move_index, my_game.current_turn);

        if my_game.is_over(&my_board) {
            break;
        } else {
            my_game.change_turn();
        }
    }

    println!("winner, winner!");
    println!("the winner is: {}", my_game.current_turn);
    my_board.pretty_print();
}
