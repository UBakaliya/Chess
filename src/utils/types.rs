#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    None,
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub piece: Piece,
    pub color: Color,
}

impl Square {
    pub fn is_occupied(&self) -> bool {
        (self.piece != Piece::None) && (self.color != Color::None)
    }
}

pub const BOARD_SIZE: usize = 8;

pub type Board = [[Square; BOARD_SIZE]; BOARD_SIZE];
