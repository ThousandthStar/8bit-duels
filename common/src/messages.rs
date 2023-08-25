use crate::card::{Card, CardEntity};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    // 1st param: whether or not the player is player_1
    StartGame(bool),
    StartTurn,
    // 1st param: the `CardEntity` to spawn
    SpawnCard(CardEntity),
    /*
    1st param: the initial x position
    2nd param: the initial y position
    3rd param: the final   x position
    4th param: the final   y position
    */
    MoveTroop(i32, i32, i32, i32),
    AttackTroop(i32, i32, i32, i32),
    EndGame(bool),
    ChatMessage(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    PlayerInfo(String, Vec<Card>),
    MoveTroop(i32, i32, i32, i32),
    AttackTroop(i32, i32, i32, i32),
    SpawnCard(Card, i32, i32),
    EndTurn,
    WinGame(i32, i32),
    ChatMessage(String),
    Resign,
}
