use crate::models::{Board, Cell};

pub fn parse_input(input: &str) -> Option<(usize, usize)> {
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
    } else if first.is_numeric() && second.is_alphabetic() {
        let row = first.to_digit(10)? as usize - 1;
        let col = second as usize - 'a' as usize;
        if row < 3 && col < 3 {
            return Some((row, col));
        }
    }

    None
}

pub fn check_win(board: &Board, player: Cell) -> bool {
    // Проверка строк
    for row in 0..3 {
        if board[row][0] == player && board[row][1] == player && board[row][2] == player {
            return true;
        }
    }

    // Проверка столбцов
    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }

    // Проверка диагоналей
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}

pub fn is_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == Cell::Empty {
                return false;
            }
        }
    }
    true
}