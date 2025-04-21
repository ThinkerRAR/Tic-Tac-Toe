#[derive(PartialEq, Copy, Clone)]
pub enum Cell {
    Empty,
    X,
    O,
}

pub type Board = [[Cell; 3]; 3];