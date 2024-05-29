use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub token: String,
}

pub fn read_game_config(path: String) -> Result<GameConfig> {
    let contents_read_attempt = fs::read_to_string(path);
    let contents = match contents_read_attempt {
        Ok(file) => file,
        Err(err) => panic!("oh no file failed to open {:?}", err),
    };
    let content_string = String::from(contents);
    let game_config: GameConfig = serde_json::from_str(&content_string)?;
    Ok(game_config)
}
