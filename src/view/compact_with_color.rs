use std::time;
use std::{
    f32,
    io::{self, Write},
};
use termion::{color, cursor, style};

use crate::{
    controller::conversion::{
        convert_fen_char_to_display, convert_piece_to_display_character_compact,
    },
    utils::Board,
};

pub fn draw_compact_board_from_fend(fen: &str) {
    let board_string = fen.to_string();
    let fen_board_and_meta_data: Vec<&str> = board_string.split(' ').collect();
    let fen_board: &str = fen_board_and_meta_data[0];

    let board: Vec<&str> = fen_board.split('/').collect();

    board.into_iter().enumerate().for_each(|(r, row)| {
        row.chars().into_iter().enumerate().for_each(|(i, fen)| {
            let total = i + r;
            if total % 2 == 0 {
                print!(
                    "{}{}",
                    color::Bg(color::Black),
                    convert_fen_char_to_display(fen)
                );
            } else {
                print!(
                    "{}{}",
                    color::Bg(color::LightWhite),
                    convert_fen_char_to_display(fen)
                );
            }
        });
        println!();
    });
    println!("ABCDEFGH");
}
fn print_square<W: Write>(handle: &mut W, is_white: bool, piece: Option<char>) {
    if is_white {
        write!(handle, "{}", color::Bg(color::LightCyan)).unwrap();
    } else {
        write!(handle, "{}", color::Bg(color::Blue)).unwrap();
    }
    write!(
        handle,
        "{} ",
        match piece {
            Some(p) => format!(
                "{}{}",
                color::Fg(color::Red),
                convert_fen_char_to_display(p)
            ),
            None => " ".to_string(),
        }
    )
    .unwrap();
}

pub fn draw_compact_board_from_board(board: &Board, white_time: i32, black_time: i32) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Clear the terminal
    write!(handle, "{}", termion::clear::All).unwrap();
    write!(handle, "{}", cursor::Goto(1, 1)).unwrap();

    let (terminal_width, _) = termion::terminal_size().unwrap();
    let white_duration = time::Duration::from_millis(white_time as u64);
    let black_duration = time::Duration::from_millis(black_time as u64);

    let spaces = (terminal_width - 29) / 2;
    print!("{}", " ".repeat(spaces as usize));
    println!(
        "White timer: {}:{} Black timer: {}:{}",
        white_duration.as_secs() / 60,
        white_duration.as_secs() % 60,
        black_duration.as_secs() / 60,
        black_duration.as_secs() % 60,
    );

    print!("{}", " ".repeat(spaces as usize));
    write!(handle, ".a b c d e f g h\n").unwrap();
    let mut is_white_square = true;

    for (i, rank) in board.pieces.iter().enumerate() {
        print!("{}", " ".repeat(spaces as usize));
        write!(handle, "{}{}", 8 - i, color::Bg(color::Reset)).unwrap();
        for square in rank {
            match square {
                None => {
                    print_square(&mut handle, is_white_square, None);
                    is_white_square = !is_white_square;
                    write!(handle, "{}", color::Bg(color::Reset)).unwrap();
                    write!(handle, "{}", color::Fg(color::Reset)).unwrap();
                }
                Some(piece) => {
                    print_square(
                        &mut handle,
                        is_white_square,
                        Some(convert_piece_to_display_character_compact(Some(*piece))),
                    );
                    is_white_square = !is_white_square;

                    write!(handle, "{}", color::Bg(color::Reset)).unwrap();
                    write!(handle, "{}", color::Fg(color::Reset)).unwrap();
                }
            }
        }
        is_white_square = !is_white_square;
        write!(handle, "\r\n").unwrap();
    }

    // Reset terminal colors
    write!(handle, "{}", color::Fg(color::Reset)).unwrap();
    write!(handle, "{}", color::Bg(color::Reset)).unwrap();
    write!(handle, "{}", style::Reset).unwrap();
}
pub fn draw_compact_board_from_fen(fen: &str) {
    let pieces: Vec<&str> = fen.split(' ').collect();
    let board = pieces[0];

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Clear the terminal
    write!(handle, "{}", termion::clear::All).unwrap();
    write!(handle, "{}", cursor::Goto(1, 1)).unwrap();
    write!(handle, ".a b c d e f g h\n").unwrap();
    let mut is_white_square = true;

    for (i, rank) in board.split('/').enumerate() {
        write!(handle, "{}{}", 8 - i, color::Bg(color::Reset)).unwrap();
        for square in rank.chars() {
            match square {
                '1'..='8' => {
                    let num_empty_squares = square.to_digit(10).unwrap() as usize;
                    for _ in 0..num_empty_squares {
                        print_square(&mut handle, is_white_square, None);
                        is_white_square = !is_white_square;
                        write!(handle, "{}", color::Bg(color::Reset)).unwrap();
                        write!(handle, "{}", color::Fg(color::Reset)).unwrap();
                    }
                }
                piece => {
                    print_square(&mut handle, is_white_square, Some(piece));
                    is_white_square = !is_white_square;

                    write!(handle, "{}", color::Bg(color::Reset)).unwrap();
                    write!(handle, "{}", color::Fg(color::Reset)).unwrap();
                }
            }
        }
        is_white_square = !is_white_square;
        write!(handle, "\r\n").unwrap();
    }

    // Reset terminal colors
    write!(handle, "{}", color::Fg(color::Reset)).unwrap();
    write!(handle, "{}", color::Bg(color::Reset)).unwrap();
    write!(handle, "{}", style::Reset).unwrap();
}
