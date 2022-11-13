use super::{QueueIn, QueueOut};
use crate::{
    tilemap::{CardSprites, TileSize},
    GameState, IsPlayer1, IsSelfTurn,
};
use bevy::prelude::*;
use common::{card::CardEntity, messages::ServerMessage};
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
    mut card_entity_q: Query<&mut CardEntity>,
) {
    let mut guard = queue_in.0.lock().unwrap();
    if let Some(message) = guard.pop_front() {
        match message {
            ServerMessage::StartGame(is_player_1) => {
                if is_player_1 {
                    commands.insert_resource(IsPlayer1(true));
                    commands.insert_resource(IsSelfTurn(true));
                } else {
                    commands.insert_resource(IsPlayer1(false));
                    commands.insert_resource(IsSelfTurn(false));
                }
                state.set(GameState::Playing);
            }
            "spawn-card" => {
                if !matches!(packet["troop"].clone(), Value::Null) {
                    let result = serde_json::from_value::<CardEntity>(packet["troop"].clone());
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
            "move-troop" => {
                let start_x = packet["start-x"].clone().as_f64().unwrap_or(f64::MAX);
                let start_y = packet["start-y"].clone().as_f64().unwrap_or(f64::MAX);
                let end_x = packet["end-x"].clone().as_f64().unwrap_or(f64::MAX);
                let end_y = packet["end-y"].clone().as_f64().unwrap_or(f64::MAX);

                for mut card_entity in card_entity_q.iter_mut() {
                    if card_entity.get_x_pos() as f64 == start_x
                        && card_entity.get_y_pos() as f64 == start_y
                    {
                        card_entity.set_x_pos(end_x);
                        card_entity.set_y_pos(end_y);
                        card_entity.moved()
                    }
                }
            }

            &_ => {}
        }
    }
}
