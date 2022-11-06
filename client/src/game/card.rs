use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::tilemap::TileSize;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_card_sprites)
            .add_system(position_sprites)
            .insert_resource(CardCollection::new());
    }
}

fn position_sprites(
    mut query: Query<(&mut Transform, &CardEntity)>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
) {
    let window = windows.get_primary().unwrap();
    // parentheses only for clarity (they are unnecessary)
    let start_y = (window.height() / 2.) - (tile_size.0 / 2.);
    let start_x = (-window.width() / 2.) + (tile_size.0 / 2.) + (window.width() / 3.);

    for (mut transform, card_entity) in query.iter_mut() {
        transform.translation.x = start_x + (card_entity.get_x_pos() as f32 * tile_size.0);
        transform.translation.y = start_y - (card_entity.get_y_pos() as f32 * tile_size.0);
        transform.translation.z = 150.;
    }
}

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

pub(crate) struct CardSprites(
    pub(crate) Handle<TextureAtlas>,
    pub(crate) HashMap<String, usize>,
);

fn load_card_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet = asset_server.load("sprite_sheet.png");
    let atlas: TextureAtlas = TextureAtlas::from_grid(sprite_sheet, Vec2::splat(8.), 1, 1);

    let atlas_handle = texture_atlases.add(atlas);
    let mut card_sprite_map = HashMap::new();
    card_sprite_map.insert("skeleton".to_string(), 0);

    commands.insert_resource(CardSprites(atlas_handle, card_sprite_map));
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

    pub(crate) fn get_damage(&self) -> f32 {
        self.attack.clone()
    }

    pub(crate) fn get_name(&self) -> String {
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
    pub(crate) card: Card,
    current_hp: f32,
    position_x: i32,
    position_y: i32,
    is_owned_by_p1: bool,
}

impl CardEntity {
    pub(crate) fn get_x_pos(&self) -> i32 {
        self.position_x.clone()
    }

    pub(crate) fn get_y_pos(&self) -> i32 {
        self.position_y.clone()
    }

    pub(crate) fn set_x_pos(&mut self, x: f64) {
        self.position_x = x as i32;
    }

    pub(crate) fn set_y_pos(&mut self, y: f64) {
        self.position_y = y as i32;
    }

    pub(crate) fn is_owned_by_p1(&self) -> bool {
        if self.is_owned_by_p1 {
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CardAbility {
    ProduceGold(i32),
}