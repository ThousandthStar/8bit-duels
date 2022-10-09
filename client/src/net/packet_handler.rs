use super::{QueueIn, QueueOut};
use crate::{
    game::card::{CardEntity, CardSprites},
    tilemap::TileSize,
    GameState,
};
use bevy::prelude::*;
use serde_json::Value;

pub(crate) struct PacketHandlerPlugin;

impl Plugin for PacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::PreparingForGame).with_system(handle_packets),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_packets));
    }
}

fn handle_packets(
    mut queue_in: ResMut<QueueIn>,
    mut queue_out: ResMut<QueueOut>,
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    card_sprites: Res<CardSprites>,
    tile_size: Res<TileSize>,
) {
    let mut guard = queue_in.0.lock().unwrap();
    if !guard.is_empty() {
        let packet_to_read = guard.pop_front().unwrap();
        let packet: Value =
            serde_json::from_str::<Value>(packet_to_read.as_str()).unwrap_or(Value::Null);
        println!("{:?}", packet);
        if matches!(packet, Value::Object(_)) {
            if let Value::String(string) = packet["packet-type"].clone() {
                println!("{:?}", string);
                match string.as_str() {
                    "server-start-game" => {
                        if matches!(packet["player-1"].clone(), Value::Bool(_bool)) {
                            let player_1: Value =
                                serde_json::from_value(packet["player-1"].clone()).unwrap();
                            commands.insert_resource(crate::IsPlayer1(
                                if player_1.as_bool().unwrap() {
                                    true
                                } else {
                                    false
                                },
                            ));
                        }
                        state.set(GameState::Playing);
                    }
                    "spawn-card" => {
                        if !matches!(packet["troop"].clone(), Value::Null) {
                            let result =
                                serde_json::from_value::<CardEntity>(packet["troop"].clone());
                            if result.is_ok() {
                                let card_entity = result.unwrap();
                                let mut sprite = TextureAtlasSprite::new(
                                    card_sprites
                                        .1
                                        .get(&card_entity.card.get_name())
                                        .unwrap()
                                        .clone(),
                                );
                                sprite.custom_size = Some(Vec2::splat(tile_size.0 * 0.8));

                                commands
                                    .spawn_bundle(SpriteSheetBundle {
                                        sprite,
                                        texture_atlas: card_sprites.0.clone(),
                                        transform: Transform::from_xyz(0., 0., 500.),
                                        ..Default::default()
                                    })
                                    .insert(card_entity);
                            }
                        }
                    }
                    "none" => {
                        println!("bad packet from the server!")
                    }
                    &_ => {}
                }
            }
        }
    }
    drop(guard);
}
