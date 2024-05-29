use rocket::futures::StreamExt;
use std::error::Error;

use crate::view::{
    big_colorless::draw_board_from_fen, compact_with_color::draw_compact_board_from_fen,
};

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub id: String,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub color: String,
    pub user: User,
    pub rating: i32,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub id: String,
    pub orientation: String,
    pub players: Vec<Player>,
    pub fen: String,
}

#[derive(Debug, Deserialize)]
pub struct TvEvent {
    pub t: String,
    pub d: Data,
}

#[derive(Debug, Deserialize)]
pub struct Fen {
    pub fen: String,
}
#[derive(Debug, Deserialize)]
struct TvBoardState {
    pub d: Fen,
}

pub async fn get_tv_feed_data() -> Result<(), Box<dyn Error>> {
    let mut stream = reqwest::get("https://lichess.org/api/tv/feed")
        .await?
        .bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        let json_result: Result<TvEvent, _> = serde_json::from_slice(&chunk);

        match json_result {
            Ok(tv_event) => {
                println!("TV Event: {:?}", tv_event);
            }
            Err(_) => {
                let json_result_board_state: Result<TvBoardState, _> =
                    serde_json::from_slice(&chunk);
                // draw_board_from_fen(&json_result_board_state.unwrap().d.fen);
                draw_compact_board_from_fen(&json_result_board_state.unwrap().d.fen);
            }
        }
    }
    Ok(())
}
