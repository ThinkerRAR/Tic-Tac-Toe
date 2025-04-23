use super::Cell;
use super::Board;



#[derive(Debug)]
pub enum ParseError {
    MoreChar,
    LessChar,
    InvalidFormat,
    UnknownChars,
}


pub fn parse_input(input: &str) -> Result<(usize, usize), ParseError> {
    let input = input.trim().to_lowercase();
    let chars: Vec<char> = input.chars().collect();

    match chars.len() {
        0 | 1 => Err(ParseError::LessChar),
        2 => {
            if !chars.iter().all(|c| "123abc".contains(*c)) {
                return Err(ParseError::UnknownChars);
            }

            if let (Some(first), Some(second)) = (chars.get(0), chars.get(1)) {
                if "123".contains(*first) && "abc".contains(*second) {
                    return Ok((
                        first.to_digit(10).unwrap() as usize - 1,
                        *second as usize - 'a' as usize
                    ));
                }

                if "abc".contains(*first) && "123".contains(*second) {
                    return Ok((
                        second.to_digit(10).unwrap() as usize - 1,
                        *first as usize - 'a' as usize
                    ));
                }
            }

            Err(ParseError::InvalidFormat)
        }
        _ => Err(ParseError::MoreChar),
    }
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