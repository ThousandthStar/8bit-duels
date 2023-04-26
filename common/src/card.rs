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
            Card::new("skeleton", CardType::Troop, 5., 3., 2, vec![]),
        );
        map.insert(
            "reaper".to_string(),
            Card::new(
                "reaper",
                CardType::Troop,
                6.,
                4.,
                5,
                vec![CardAbility::SpiritCollector],
            ),
        );
        map.insert(
            "kraken".to_string(),
            Card::new(
                "kraken",
                CardType::Troop,
                12.,
                1.,
                5,
                vec![CardAbility::MultiAttack {
                    max_attacks: 2_u8,
                    attack_count: 0_u8,
                }],
            ),
        );
        map.insert(
            "spider".to_string(),
            Card::new(
                "spider",
                CardType::Troop,
                4.,
                2.,
                4,
                vec![CardAbility::Stun { amount: 2 }],
            ),
        );
        CardCollection(map)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

    pub fn get_cost(&self) -> i32 {
        self.cost.clone()
    }

    pub fn get_abilities(&self) -> Vec<CardAbility> {
        self.abilities.clone()
    }

    pub fn get_abilities_mut(&mut self) -> &mut Vec<CardAbility> {
        &mut self.abilities
    }
}

impl From<&str> for Card{
    fn from(value: &str) -> Self {
        let collection = CardCollection::new();
        collection.0.get(&value.to_string()).unwrap_or(collection.0.get(&"skeleton".to_owned()).unwrap()).clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum CardType {
    Troop,
    Spell,
    Building,
}

#[derive(Serialize, Deserialize, Clone, Debug, Component, PartialEq)]
pub struct CardEntity {
    card: Card,
    pub current_hp: f32,
    position_x: i32,
    position_y: i32,
    is_owned_by_p1: bool,
    has_moved: bool,
    has_attacked: bool,
    pub stun_count: i32,
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
            stun_count: 1,
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
        let mut card = &mut self.card;
        for ability in card.get_abilities_mut() {
            if let CardAbility::MultiAttack {
                ref mut max_attacks,
                ref mut attack_count,
            } = ability
            {
                if attack_count < max_attacks {
                    *attack_count += 1;
                    return;
                }
            }
        }
        self.has_attacked = true;
    }

    pub fn get_card(&self) -> Card {
        self.card.clone()
    }

    pub fn reset(&mut self) {
        self.has_moved = false;
        self.has_attacked = false;
        if self.stun_count > 0 {
            self.stun_count -= 1;
        }
        let mut card = &mut self.card;
        for ability in card.get_abilities_mut() {
            if let CardAbility::MultiAttack {
                max_attacks: _,
                ref mut attack_count,
            } = ability
            {
                *attack_count = 0_u8;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CardAbility {
    MultiAttack { max_attacks: u8, attack_count: u8 },
    SpiritCollector,
    Stun { amount: i32 },
}
