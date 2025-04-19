use std::io;

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    X,
    O,
}

fn main() {
    println!("Крестики-нолики (игра против бота)");
    println!("Ты играешь крестиками (X)");
    println!("Вводи ходы в формате 'a1', 'b2' и т.д.");

    let mut board = [
        [Cell::Empty, Cell::Empty, Cell::Empty],
        [Cell::Empty, Cell::Empty, Cell::Empty],
        [Cell::Empty, Cell::Empty, Cell::Empty],
    ];

    loop {

        show_board(&board);


        println!("Твой ход (X):");
        
        let pos = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Ошибка чтения");
            
            match parse_input(&input) {
                Some(pos) if board[pos.0][pos.1] == Cell::Empty => break pos,
                Some(_) => println!("Клетка занята! Попробуй ещё:"),
                None => println!("Неправильный формат! Введи типа 'a1':"),
            }
        };


        board[pos.0][pos.1] = Cell::X;


        if check_win(&board, Cell::X) {
            show_board(&board);
            println!("Ты победил! 🎉");
            break;
        }


        if is_draw(&board) {
            show_board(&board);
            println!("Ничья! 🤝");
            break;
        }

    }
}

fn show_board(board: &[[Cell; 3]; 3]) {
    println!("   a   b   c");
    println!(" +---+---+---+");
    for (i, row) in board.iter().enumerate() {
        print!("{}|", i + 1);
        for cell in row {
            print!(" {} |", match cell {
                Cell::X => "X",
                Cell::O => "O",
                Cell::Empty => " ",
            });
        }
        println!();
        println!(" +---+---+---+");
    }
}


fn parse_input(input: &str) -> Option<(usize, usize)> {
    let input = input.trim().to_lowercase();
    if input.len() != 2 {
        return None;
    }

    let mut chars = input.chars();
    let first = chars.next()?;
    let second = chars.next()?;


    if first.is_alphabetic() && second.is_numeric() {
        let col = first as usize - 'a' as usize;
        let row = second.to_digit(10)? as usize - 1;
        if row < 3 && col < 3 {
            return Some((row, col));
        }
    }

    else if first.is_numeric() && second.is_alphabetic() {
        let row = first.to_digit(10)? as usize - 1;
        let col = second as usize - 'a' as usize;
        if row < 3 && col < 3 {
            return Some((row, col));
        }
    }

    None
}

fn check_win(board: &[[Cell; 3]; 3], player: Cell) -> bool {

    for row in 0..3 {
        if board[row][0] == player && board[row][1] == player && board[row][2] == player {
            return true;
        }
    }


    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }


    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}


fn is_draw(board: &[[Cell; 3]; 3]) -> bool {
    for row in board {
        for cell in row {
            if *cell == Cell::Empty {
                return false;
            }
        }
    }
    true
}