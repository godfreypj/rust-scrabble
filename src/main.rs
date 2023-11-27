// main.rs

mod board;

use board::board::Board;

fn main() {
    let mut scrabble_board = Board::new();

    // Display the initial state of the board
    scrabble_board.display();

    // Place a word on the board starting from index 3
    if let Err(err) = scrabble_board.place_word("RUST", 3) {
        println!("Error: {}", err);
    }

    // Display the updated state of the board
    scrabble_board.display();
}