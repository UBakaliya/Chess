use crate::check_moves::*;
use crate::types::*;
use rand::Rng;
use std::collections::HashMap;

use super::display;

fn create_empty_board() -> Board {
    [[Square::new(Piece::None, Color::None); BOARD_SIZE]; BOARD_SIZE]
}

pub fn reset_board(board: &mut Board) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = Square::new(Piece::None, Color::None);
        }
    }
}

fn get_black_piece(piece: Piece) -> Option<char> {
    let black_piece: HashMap<Piece, char> = [
        (Piece::King, '\u{265A}'),
        (Piece::Queen, '\u{265B}'),
        (Piece::Rook, '\u{265C}'),
        (Piece::Bishop, '\u{265D}'),
        (Piece::Knight, '\u{265E}'),
        (Piece::Pawn, '\u{265F}'),
    ]
    .iter()
    .cloned()
    .collect();

    black_piece.get(&piece).cloned()
}

fn get_white_piece(piece: Piece) -> Option<char> {
    let white_piece: HashMap<Piece, char> = [
        (Piece::King, '\u{2654}'),
        (Piece::Queen, '\u{2655}'),
        (Piece::Rook, '\u{2656}'),
        (Piece::Bishop, '\u{2657}'),
        (Piece::Knight, '\u{2658}'),
        (Piece::Pawn, '\u{2659}'),
    ]
    .iter()
    .cloned()
    .collect();

    white_piece.get(&piece).cloned()
}

/**
 * Get a piece on the board this will be help full for getting
 * pieces when displaying pieces on the board
 */
pub fn get_piece(board_piece: Square) -> Option<char> {
    match board_piece.color {
        Color::White => get_black_piece(board_piece.piece),
        _ => get_white_piece(board_piece.piece),
    }
}

/**
 * Place the piece on board at its positions like a
 * slandered chess board has pieces on board
 */
pub fn set_table() -> Board {
    let mut board = create_empty_board();

    let pieces = vec![
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Queen,
        Piece::King,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook,
    ];

    // place white pawns
    for i in 0..BOARD_SIZE {
        board[0][i] = Square::new(pieces[i], Color::Black);
        board[1][i] = Square::new(Piece::Pawn, Color::Black);
        board[6][i] = Square::new(Piece::Pawn, Color::White);
        board[7][i] = Square::new(pieces[i], Color::Black);
    }

    board
}

/**
 * Make the move on the given fromX,fromY - toX,toY
 * Validate the move and check if the move is valid if valid
 * then move the piece
 */
pub fn move_piece(fromX: char, fromY: i32, toX: char, toY: i32) -> bool {
    true
}



