use std::{convert::TryInto, usize};

use crate::controller::conversion::{convert_char_cord_to_index, convert_num_cord_to_index};

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: Vec<Vec<Option<Piece>>>,
}

enum CastleMove {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}
fn move_piece(
    mut board: Board,
    source_c: usize,
    source_n: usize,
    dest_c: usize,
    dest_n: usize,
    is_castle: Option<CastleMove>,
) -> Board {
    let temp_storage = board.pieces[source_c][source_n].clone();
    board.pieces[source_c][source_n] = None;
    board.pieces[dest_c][dest_n] = temp_storage;
    if let Some(castle) = is_castle {
        match castle {
            CastleMove::WhiteKingside => {
                //h1f1
                board.pieces[7][5] = board.pieces[7][7];
                board.pieces[7][7] = None;
            }
            CastleMove::WhiteQueenside => {
                board.pieces[7][3] = board.pieces[7][0];
                board.pieces[7][0] = None;
            }
            CastleMove::BlackKingside => {
                board.pieces[0][5] = board.pieces[0][7];
                board.pieces[0][7] = None;
            }
            CastleMove::BlackQueenside => {
                board.pieces[0][3] = board.pieces[0][0];
                board.pieces[0][0] = None;
            }
        }
    }
    return board;
}

fn create_board_from_fen(fen: &str) -> Board {
    let fen_board: Vec<&str> = fen.split('/').collect();
    let new_board_buffer: Vec<Vec<Option<Piece>>> = fen_board
        .iter()
        .map(|row| {
            row.chars()
                .take_while(|&c| !c.is_whitespace())
                .map(convert_fen_char_to_piece)
                .flatten()
                .collect::<Vec<Option<Piece>>>()
        })
        .collect();

    Board {
        pieces: new_board_buffer,
    }
}
pub fn process_uci_to_board(moves: &str) -> Board {
    let mut board: Board = initial_board();
    let moves_vec: Vec<&str> = moves.split_whitespace().collect();
    for mv in moves_vec.clone() {
        let mut chars = mv.chars();
        let source_n = convert_char_cord_to_index(chars.next());
        let source_c = convert_num_cord_to_index(chars.next());
        let dest_n = convert_char_cord_to_index(chars.next());
        let dest_c = convert_num_cord_to_index(chars.next());
        let is_castle = is_move_castle_move(mv);

        board = move_piece(board, source_c, source_n, dest_c, dest_n, is_castle);
    }
    return board;
}

fn is_move_castle_move(uci_move: &str) -> Option<CastleMove> {
    match uci_move {
        "e1g1" => Some(CastleMove::WhiteKingside), // White kingside castling
        "e8g8" => Some(CastleMove::BlackKingside), // Black kingside castling
        "e1c1" => Some(CastleMove::WhiteQueenside), // White queenside castling
        "e8c8" => Some(CastleMove::BlackQueenside), // Black queenside castling
        _ => None,
    }
}
fn initial_board() -> Board {
    create_board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
}
pub fn convert_fen_char_to_piece(fen_char: char) -> Vec<Option<Piece>> {
    let piece = match fen_char {
        'p' => Some(Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::Black,
        }),
        'r' => Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        }),
        'n' => Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::Black,
        }),
        'b' => Some(Piece {
            kind: PieceKind::Bishop,
            color: PieceColor::Black,
        }),
        'q' => Some(Piece {
            kind: PieceKind::Queen,
            color: PieceColor::Black,
        }),
        'k' => Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::Black,
        }),
        'P' => Some(Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::White,
        }),
        'R' => Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        }),
        'N' => Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        }),
        'B' => Some(Piece {
            kind: PieceKind::Bishop,
            color: PieceColor::White,
        }),
        'Q' => Some(Piece {
            kind: PieceKind::Queen,
            color: PieceColor::White,
        }),
        'K' => Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        }),
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => None,
        _ => panic!("Recieved invalid fen character"),
    };
    match piece {
        None => {
            let quantity_of_empty_tile: usize = fen_char.to_digit(10).unwrap().try_into().unwrap();
            let my_vector: Vec<Option<Piece>> = (0..quantity_of_empty_tile).map(|_| None).collect();
            my_vector
        }
        Some(_) => {
            vec![piece]
        }
    }
}


pub fn convert_piece_to_display_character(piece: Option<Piece>) -> String {
    match piece {
        Some(Piece { color, kind }) => match (color, kind) {
            (PieceColor::Black, PieceKind::Pawn) => "┃ ♙ ".to_string(),
            (PieceColor::Black, PieceKind::Knight) => "┃ ♘ ".to_string(),
            (PieceColor::Black, PieceKind::Bishop) => "┃ ♗ ".to_string(),
            (PieceColor::Black, PieceKind::Rook) => "┃ ♖ ".to_string(),
            (PieceColor::Black, PieceKind::Queen) => "┃ ♕ ".to_string(),
            (PieceColor::Black, PieceKind::King) => "┃ ♔ ".to_string(),

            (PieceColor::White, PieceKind::Pawn) => "┃ ♟ ".to_string(),
            (PieceColor::White, PieceKind::Knight) => "┃ ♞ ".to_string(),
            (PieceColor::White, PieceKind::Bishop) => "┃ ♝ ".to_string(),
            (PieceColor::White, PieceKind::Rook) => "┃ ♜ ".to_string(),
            (PieceColor::White, PieceKind::Queen) => "┃ ♛ ".to_string(),
            (PieceColor::White, PieceKind::King) => "┃ ♚ ".to_string(),
        },
        None => "┃   ".to_string(),
    }
}
