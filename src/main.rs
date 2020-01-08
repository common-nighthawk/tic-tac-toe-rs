pub mod board;
pub mod game;

fn main() {
    let mut my_game = game::new_game();
    let mut my_board = board::new_board();

    while !my_game.is_over(&my_board) {
        println!("current game board--");
        my_board.pretty_print();

        println!("next move is: {}", my_game.current_turn);
        println!("where do you want to go?");
        let move_index = game::move_request();

        my_board.update_square(move_index, my_game.current_turn);
        my_game.change_turn();
    }

    println!("");
    println!("winner, winner!");
    println!("the winner is: {}", my_game.winner());
    my_board.pretty_print();
}
