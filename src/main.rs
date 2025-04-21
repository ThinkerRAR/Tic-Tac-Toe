mod models;
mod display;
mod game;

use std::io;
use crate::models::{Board, Cell};
use crate::display::show_board;
use crate::game::{parse_input, check_win, is_draw};

fn main() {
    println!("Крестики-нолики (игра против бота)");
    println!("Ты играешь крестиками (X)");
    println!("Вводи ходы в формате 'a1', 'b2' и т.д.");

    let mut board: Board = [[Cell::Empty; 3]; 3];

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