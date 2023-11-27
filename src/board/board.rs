// board.rs

// Module: The Board
// Description: The Scrabble board to be solved. This is immutable and always
// the same for every instance of our Agent. In fact, it's not even a whole
// scrabble board because why waste the memory? The starting rack is 7 tiles,
// the first move can only be horizontal, so the board is just the middle 13
// squares of a standard Scrabble Board.

/// The Board struct represents the Scrabble game board and provides methods
/// to interact with and manipulate the board.
pub struct Board {
    spaces: Vec<Option<char>>, // Use Option<char> to represent an empty space or a space with a letter
}

impl Board {
    pub fn new() -> Self {
        // Initialize a new board
        let mut spaces = Vec::with_capacity(13);
        // with empty spaces
        spaces.resize(13, None);
        // create a new instance
        Board { spaces }
    }

    pub fn display(&self) {
        // Print a blank line
        print!("\n\n\n---- Scrabble ----\n");
        // Display the current state of the board
        for space in &self.spaces {
            match space {
                Some(letter) => print!(" {} ", letter),
                None => print!("_"),
            }
        }
        // Print a blank line
        print!("\n---------------");
    }

    pub fn place_word(&mut self, word: &str, starting_cell: usize) -> Result<(), String> {
        // Given the starting cell, place the word on the board

        // Split the word into characters
        let word_chars: Vec<char> = word.chars().collect();

        // Place the word on the board
        for (i, &ch) in word_chars.iter().enumerate() {
            self.spaces[starting_cell + i] = Some(ch);
        }

        Ok(())
    }
}