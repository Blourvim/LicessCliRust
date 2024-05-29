use crate::utils::{Piece, PieceColor, PieceKind};

pub fn convert_char_cord_to_index(c: Option<char>) -> usize {
    if let Some(item) = c {
        match item {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!(
                "recieved unknown cords char for convert_alphabet_to_index, {:?}",
                c
            ),
        }
    } else {
        panic!("recieved unknown cords char for convert_alphabet_to_index");
    }
}
pub fn convert_num_cord_to_index(n: Option<char>) -> usize {
    if let Some(item) = n {
        let digit = item.to_digit(10);
        (8 - digit.expect("recieved unknown cords char for convert_alphabet_to_index")) as usize
    } else {
        panic!("found bad move")
    }
}
pub fn convert_fen_char_to_display(fen: char) -> char {
    match fen {
        'K' => '♚',
        'Q' => '♛',
        'R' => '♜',
        'B' => '♝',
        'N' => '♞',
        'P' => '♟',

        'k' => '♔',
        'q' => '♕',
        'r' => '♖',
        'b' => '♗',
        'n' => '♘',
        'p' => '♙',

        _ => fen,
    }
}

pub fn convert_piece_to_display_character_compact(piece: Option<Piece>) -> char {
    match piece {
        Some(Piece { color, kind }) => match (color, kind) {
            (PieceColor::White, PieceKind::Pawn) => '♙',
            (PieceColor::White, PieceKind::Knight) => '♘',
            (PieceColor::White, PieceKind::Bishop) => '♗',
            (PieceColor::White, PieceKind::Rook) => '♖',
            (PieceColor::White, PieceKind::Queen) => '♕',
            (PieceColor::White, PieceKind::King) => '♔',

            (PieceColor::Black, PieceKind::Pawn) => '♟',
            (PieceColor::Black, PieceKind::Knight) => '♞',
            (PieceColor::Black, PieceKind::Bishop) => '♝',
            (PieceColor::Black, PieceKind::Rook) => '♜',
            (PieceColor::Black, PieceKind::Queen) => '♛',
            (PieceColor::Black, PieceKind::King) => '♚',
        },
        None => ' ',
    }
}
