use super::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerDeck {
    deck: Vec<Card>,
}
