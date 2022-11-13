use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct CardCollection(pub HashMap<String, Card>);

impl CardCollection {
    pub fn init() -> CardCollection {
        let mut map: HashMap<String, Card> = HashMap::new();
        map.insert(
            "skeleton".to_string(),
            Card::new("skeleton", CardType::TROOP, 5., 5., 10, vec![]),
        );
        map.insert(
            "gold-mine".to_string(),
            Card::new(
                "gold-mine",
                CardType::BUILDING,
                20.,
                0.,
                35,
                vec![CardAbility::ProduceGold(5)],
            ),
        );
        CardCollection(map)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    name: String,
    type_: CardType,
    hp: f32,
    attack: f32,
    cost: i32,
    abilities: Vec<CardAbility>,
}

impl Card {
    fn new(
        name: &str,
        type_: CardType,
        hp: f32,
        attack: f32,
        cost: i32,
        abilities: Vec<CardAbility>,
    ) -> Card {
        Card {
            name: name.to_string(),
            type_,
            hp,
            attack,
            cost,
            abilities,
        }
    }

    pub fn get_hp(&self) -> f32 {
        self.hp.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CardType {
    TROOP,
    SPELL,
    BUILDING,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CardEntity {
    card: Card,
    current_hp: f32,
    position_x: i32,
    position_y: i32,
    pub(crate) is_owned_by_p1: bool,
    pub(crate) has_moved: bool,
    pub(crate) has_attacked: bool,
}

impl CardEntity {
    pub(crate) fn new(
        card: &Card,
        position_x: i32,
        position_y: i32,
        is_owned_by_p1: bool,
    ) -> CardEntity {
        CardEntity {
            card: card.clone(),
            current_hp: card.get_hp(),
            position_x,
            position_y,
            is_owned_by_p1,
            has_moved: false,
            has_attacked: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CardAbility {
    ProduceGold(i32),
}
