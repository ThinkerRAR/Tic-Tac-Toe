mod display;
mod game;
mod cell;

use cell::{Cell, Board};
use std::io;
use display::show_board;
use game::{parse_input, check_win, is_draw, ParseError};



fn main() {
    println!("Крестики-нолики (игра против бота)");
    println!("Ты играешь крестиками (X)");
    println!("Вводи ходы в формате 'a1', 'b2' и т.д.");

    let mut board: Board = [[Cell::Empty; 3]; 3];

    loop {
        show_board(board);

        println!("Твой ход (X):");
        
        let pos = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Ошибка чтения");
            
            match parse_input(&input) {
                Ok(pos) if board[pos.0][pos.1] == Cell::Empty => break pos,
                Ok(_) => println!("Клетка занята! Попробуй ещё:"),
                Err(ParseError::LessChar) => println!("Плохо, камбой, ты ввёл меньше 2 символов"),
                Err(ParseError::MoreChar) => println!("Воу, камбой, ты ввёл больше 2 символов"),
                Err(ParseError::InvalidFormat) => println!("Неправильный формат! Введи типа 'a1' или '1a'"),
                Err(ParseError::UnknownChars) => println!("Недопустимые символы! Используй только a,b,c и 1,2,3"),
            }
        };

        board[pos.0][pos.1] = Cell::X;

        if check_win(&board, Cell::X) {
            show_board(board);
            println!("Ты победил!");
            break;
        }

        if is_draw(&board) {
            show_board(board);
            println!("Ничья!");
            break;
        }
    }
}