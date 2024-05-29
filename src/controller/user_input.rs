use crate::apis::{
    board_api::{board_game_move, board_game_resign},
    configuration::Configuration,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc::Receiver,
};

pub async fn input_handler(configuration: Configuration, mut game_id_receiver: Receiver<String>) {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);

    if let Some(message) = game_id_receiver.recv().await {
        println!("game id is: {:?}", message);
        loop {
            let mut line = String::new();
            if let Ok(bytes_read) = reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                if line.trim() == "ff" {
                    print!("surrendering,{:?}", line);
                    let _ = board_game_resign(&configuration, &message).await;
                } else {
                    let _ = board_game_move(&configuration, &message, &line, None).await;
                }
                println!("User input: {}", line.trim());
            } else {
                eprintln!("Error reading user input");
                break;
            }
        }
    }
}
