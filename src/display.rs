use super::Cell;

pub fn show_board(board: [[Cell; 3]; 3]) {
    println!("\x1B[2J\x1B[1;1H   a   b   c");

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