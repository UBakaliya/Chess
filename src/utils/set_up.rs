use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    None,
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoardPiece {
    piece: Piece,
    color: Color,
}

const BOARD_SIZE: usize = 8;

type Board = [[BoardPiece; BOARD_SIZE]; BOARD_SIZE];

pub fn create_empty_board() -> Board {
    let board: Board = [[BoardPiece {
        piece: Piece::None,
        color: Color::None,
    }; BOARD_SIZE]; BOARD_SIZE];

    board
}

pub fn reset_board(board: &mut Board) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = BoardPiece {
                piece: Piece::None,
                color: Color::None,
            };
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

pub fn get_piece(board_piece: BoardPiece) -> Option<char> {
    if board_piece.color == Color::White {
        return get_black_piece(board_piece.piece);
    }

    get_white_piece(board_piece.piece)
}

pub fn build_table(board: &mut Board) {
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

    for i in 0..BOARD_SIZE {
        board[0][i] = BoardPiece {
            piece: pieces[i],
            color: Color::Black,
        };
    }

    // place black pawns
    for i in 0..BOARD_SIZE {
        board[1][i] = BoardPiece {
            piece: Piece::Pawn,
            color: Color::Black,
        };
    }

    // place white pawns
    for i in 0..BOARD_SIZE {
        board[6][i] = BoardPiece {
            piece: Piece::Pawn,
            color: Color::White,
        };
    }

    for i in 0..BOARD_SIZE {
        board[7][i] = BoardPiece {
            piece: pieces[i],
            color: Color::Black,
        };
    }
}

pub fn pick_random_player() -> i32 {
    rand::thread_rng().gen_range(1..=2)
}
