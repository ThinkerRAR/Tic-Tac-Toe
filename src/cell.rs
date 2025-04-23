#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::X => "X",
            Cell::O => "O",
            Cell::Empty => " ",
        })
    }
}

pub type Board = [[Cell; 3]; 3];