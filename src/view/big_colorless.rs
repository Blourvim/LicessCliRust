use std::usize;

use crate::{
    controller::conversion::convert_fen_char_to_display,
    utils::{convert_fen_char_to_piece, convert_piece_to_display_character, Board, Piece},
};
pub fn draw_board_from_board(board: &Board) {
    println!("board{:?}", board);
    println!("┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓");
    board
        .pieces
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(r, line)| {
            line.clone().into_iter().enumerate().for_each(|(i, piece)| {
                print!("{}", convert_piece_to_display_character(piece));
                if i == 7 {
                    println!("┃{}", 9 - (r + 1))
                }
            });
            if r != 7 {
                println!("┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫");
            }
        });

    println!("┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛",);
    println!("  A   B   C   D   E   F   G   H  ");
}

pub fn draw_board_from_fen(fen: &str) {
    let board_string = fen.to_string();
    let fen_board_and_meta_data: Vec<&str> = board_string.split(' ').collect();
    let fen_board: &str = fen_board_and_meta_data[0];

    let board: Vec<&str> = fen_board.split('/').collect();

    println!("┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓");
    board.into_iter().enumerate().for_each(|(r, row)| {
        row.chars().into_iter().for_each(|fen| {
            if let Some(digit) = fen.to_digit(10) {
                print!("{}", "┃   ".repeat(digit as usize));
            } else {
                print!("┃ {} ", convert_fen_char_to_display(fen));
            }
        });

        println!("┃{}", 9 - (r + 1));
        if r != 7 {
            println!("┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫");
        }
    });
    println!("┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛",);
    println!("  A   B   C   D   E   F   G   H  ");
}
pub fn draw_board_from_fen_deprecated(fen: &str) {
    let board_string = fen.to_string();
    let board: Vec<&str> = board_string.split('/').collect();

    println!("┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓");
    for (r, row) in board.into_iter().enumerate() {
        let mut pieces_of_row: Vec<Vec<Option<Piece>>> = vec![];
        for fen_char in row.to_string().chars() {
            if fen_char.is_whitespace() {
                break;
            }
            pieces_of_row.push(convert_fen_char_to_piece(fen_char));
        }
        let pieces: Vec<Option<Piece>> = pieces_of_row.into_iter().flatten().collect();
        for (i, piece) in pieces.into_iter().enumerate() {
            print!("{}", convert_piece_to_display_character(piece),);
            if i == 7 {
                print!("┃{}", r + 1)
            }
        }
        if r != 7 {
            println!("┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫");
        } else {
            println!("┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛",);
            println!("  A   B   C   D   E   F   G   H");
        }
    }
}
