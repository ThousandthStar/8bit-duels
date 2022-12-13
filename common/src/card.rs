use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Resource)]
pub struct CardCollection(pub HashMap<String, Card>);

impl CardCollection {
    pub fn new() -> CardCollection {
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
    pub name: String,
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

    pub fn get_damage(&self) -> f32 {
        self.attack.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CardType {
    TROOP,
    SPELL,
    BUILDING,
}

#[derive(Serialize, Deserialize, Clone, Debug, Component)]
pub struct CardEntity {
    card: Card,
    pub current_hp: f32,
    position_x: i32,
    position_y: i32,
    is_owned_by_p1: bool,
    has_moved: bool,
    has_attacked: bool,
}

impl CardEntity {
    pub fn new(card: &Card, position_x: i32, position_y: i32, is_owned_by_p1: bool) -> CardEntity {
        CardEntity {
            card: card.clone(),
            position_x,
            position_y,
            is_owned_by_p1,
            current_hp: card.hp,
            has_moved: false,
            has_attacked: false,
        }
    }

    pub fn get_x_pos(&self) -> i32 {
        self.position_x.clone()
    }

    pub fn get_y_pos(&self) -> i32 {
        self.position_y.clone()
    }

    pub fn set_x_pos(&mut self, x: i32) {
        self.position_x = x;
    }

    pub fn set_y_pos(&mut self, y: i32) {
        self.position_y = y;
    }

    pub fn is_owned_by_p1(&self) -> bool {
        if self.is_owned_by_p1 {
            true
        } else {
            false
        }
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved.clone()
    }

    pub fn has_attacked(&self) -> bool {
        self.has_attacked.clone()
    }

    pub fn moved(&mut self) {
        self.has_moved = true;
    }

    pub fn attacked(&mut self) {
        self.has_attacked = true;
    }

    pub fn get_card(&self) -> Card {
        self.card.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CardAbility {
    ProduceGold(i32),
}
