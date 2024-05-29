use my_lib::{
    apis::{account_api::account_me, configuration::Configuration},
    controller::user_input::input_handler,
    models::{ChatLineEvent, GameFullEvent, GameStateEvent, OpponentGone, UserExtended},
    read_config::read_game_config,
    tv::get_tv_feed_data,
    utils::process_uci_to_board,
    view::{
        big_colorless::draw_board_from_board, compact_with_color::draw_compact_board_from_board,
    },
};
use reqwest::Client;

use rocket::futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use tokio::sync::mpsc::{self, Sender};
use tokio::time::Duration;
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum BoardStream {
    #[serde(rename = "gameFull")]
    GameFullEvent(GameFullEvent),
    #[serde(rename = "gameState")]
    GameStateEvent(GameStateEvent),
    #[serde(rename = "chatLine")]
    ChatLineEvent(ChatLineEvent),
    #[serde(rename = "opponentGone")]
    OpponentGone(OpponentGone),
}
async fn fetch_self(configuration: &Configuration) -> UserExtended {
    let my_profile = account_me(configuration).await;
    match my_profile {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            panic!("heck, user fail");
        }
    }
}
async fn stream_board(client: &Client, id: &str, token: &str) -> Result<(), Box<dyn Error>> {
    println!("streaming board");
    let mut stream = client
        .get(format!("https://lichess.org/api/board/game/stream/{}", id))
        .bearer_auth(token)
        .send()
        .await?
        .bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        if chunk.len() > 2 {
            let json_result: Result<BoardStream, _> = serde_json::from_slice(&chunk);
            match json_result {
                Ok(item) => match item {
                    BoardStream::GameFullEvent(event) => println!("{:?}", event.initial_fen),
                    BoardStream::GameStateEvent(event) => {
                        println!("event: {:?}", event);
                        if let Some(val) = event.winner {
                            println!("---------------Winner {:?}----------------", val)
                        }
                        let new_board = process_uci_to_board(&event.moves);
                        draw_compact_board_from_board(&new_board,event.wtime,event.btime)
                        // draw_board_from_board(&new_board);
                    }
                    BoardStream::ChatLineEvent(_) => {}
                    BoardStream::OpponentGone(event) => {
                        if event.claim_win_in_seconds == Some(0.0) {
                            let _ = client
                                .post(format!(
                                    "https://lichess.org/api/board/game/{}/claim-victory",
                                    id
                                ))
                                .bearer_auth(token)
                                .send()
                                .await;
                        }
                        println!("{:?}", event)
                    }
                },
                Err(err) => {
                    println!("{:?}", chunk);
                    eprintln!("catto {:?}", err)
                }
            };
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum GameEvent {
    GameStart,         // Start of a game
    GameFinish,        // Completion of a game
    Challenge,         // A player sends you a challenge or you challenge someone
    ChallengeCanceled, // A player cancels their challenge to you
    ChallengeDeclined, // The opponent declines your challenge
}
async fn game_event_handler(event: &Option<GameEvent>) {
    println!("Game Event: {:?}", event);
    match event {
        Some(item) => match item {
            GameEvent::GameStart => {
                println!("game started");
            }
            GameEvent::GameFinish => todo!(),
            GameEvent::Challenge => todo!(),
            GameEvent::ChallengeCanceled => todo!(),
            GameEvent::ChallengeDeclined => todo!(),
        },
        None => {
            println!("no game");
        }
    };
}
async fn game_event_listener(
    token: &str,
    client: &Client,
    game_id_sender: Sender<String>,
) -> Result<Option<GameEvent>, Box<dyn Error>> {
    let mut stream = client
        .get("https://lichess.org/api/stream/event")
        .bearer_auth(token)
        .send()
        .await?
        .bytes_stream();

    println!("listening to events");
    while let Some(item) = stream.next().await {
        let chunk = item?;
        let game_event: Option<GameEvent> = if chunk.len() > 1 {
            let event_response: serde_json::Value = serde_json::from_slice(&chunk)?;
            if event_response["type"] == "gameStart" {
                println!("game start event {:?}", event_response);
                let game_id = &event_response["game"]["fullId"].to_string();
                let cleaned_id: String = game_id
                    .chars()
                    .filter(|&c| c.is_ascii_alphanumeric())
                    .collect();
                game_id_sender
                    .send(cleaned_id.clone())
                    .await
                    .expect("oh no game id fell");
                println!("gonna stream the game ");
                let _ = stream_board(client, &cleaned_id, token).await;

                // use the game id and start displaying the game moves
                return Ok(Some(GameEvent::GameStart));
            } else if event_response["type"] == "gameFinish" {
                return Ok(Some(GameEvent::GameFinish));
            } else if event_response["type"] == "challenge" {
                return Ok(Some(GameEvent::Challenge));
            } else if event_response["type"] == "challengeCanceled" {
                return Ok(Some(GameEvent::ChallengeCanceled));
            } else if event_response["type"] == "ChallengeDeclined" {
                return Ok(Some(GameEvent::ChallengeDeclined));
            } else {
                None
            }
        } else {
            None
        };

        game_event_handler(&game_event).await;
    }

    return Ok(None);
}
async fn start_a_match_seek(token: &String, client: &Client) -> Result<(), Box<dyn Error>> {
    let mut params = HashMap::new();
    params.insert("rated", "true");
    params.insert("time", "10");
    params.insert("increment", "0");
    println!("starting game seek");

    tokio::time::sleep(Duration::from_secs(2)).await;
    let mut response = client
        .post("https://lichess.org/api/board/seek")
        .bearer_auth(token)
        .form(&params)
        .send()
        .await?
        .bytes_stream();
    while let Some(item) = response.next().await {
        println!("Chunk: {:?}", item?);
    }
    println!("found game");

    Ok(())
}
#[tokio::main]
async fn main() {
    let my_client = Client::new();
    let (game_id_sender, game_id_receiver) = mpsc::channel::<String>(1);
    let game_config =
        read_game_config("/home/dorian/Documents/personal/rschess/config.json".to_string());
    let token: String = match game_config {
        Ok(val) => val.token.to_string(),
        _ => panic!("error with reading config"),
    };
    let configuration = Configuration {
        base_path: "https://lichess.org".to_string(),
        user_agent: None,
        client: my_client.clone(),
        basic_auth: None,
        oauth_access_token: Some(token.clone()),
        bearer_access_token: None,
        api_key: None,
    };
    tokio::task::spawn(input_handler(configuration.clone(), game_id_receiver));
    let result = fetch_self(&configuration).await;
    println!("Welcome {}", result.username.unwrap().unwrap());
    // check if in a current game
    println!("{:?}",2+2);
    let _ = tokio::join!(
        game_event_listener(&token, &my_client, game_id_sender),
        start_a_match_seek(&token, &my_client),
        // get_tv_feed_data()
    );
}
