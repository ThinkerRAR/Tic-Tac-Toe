use crate::models::{Board, Cell};

pub fn show_board(board: &Board) {
    println!("   a   b   c");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i + 1);
        for (j, cell) in row.iter().enumerate() {
            print!(" {} ", match cell {
                Cell::X => "X",
                Cell::O => "O",
                Cell::Empty => " ",
            });
            if j < 2 {
                print!("|");
            }
        }
        println!();
        if i < 2 {
            println!("  -----------");
        }
    }
}