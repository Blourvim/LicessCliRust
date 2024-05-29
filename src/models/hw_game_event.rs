use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameEventInfo {
    pub id: String,
    pub source: Source,
    pub status: Status,
    pub winner: Winner,
    pub compat: Compatibility,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    Lobby,
    Friend,
    Ai,
    Api,
    Tournament,
    Position,
    Import,
    ImportLive,
    Simul,
    Relay,
    Pool,
    Swiss,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: u8,
    pub name: GameStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum GameStatus {
    Created,
    Started,
    Aborted,
    Mate,
    Resign,
    Stalemate,
    Timeout,
    Draw,
    OutOfTime,
    Cheat,
    NoStart,
    UnknownFinish,
    VariantEnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Winner {
    White,
    Black,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compatibility {
    pub bot: bool,
    pub board: bool,
}
