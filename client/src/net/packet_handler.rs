use super::{QueueIn, QueueOut};
use crate::{
    tilemap::{CardSprites, TileSize},
    GameState, IsPlayer1, IsSelfTurn,
};
use bevy::prelude::*;
use common::{card::CardEntity, messages::ServerMessage};

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
    mut card_entity_q: Query<(Entity, &mut CardEntity)>,
    mut is_self_turn: ResMut<IsSelfTurn>,
    mut is_player_1_res: ResMut<IsPlayer1>,
) {
    let mut guard = queue_in.0.lock().unwrap();
    if let Some(message) = guard.pop_front() {
        match message {
            ServerMessage::StartGame(is_player_1) => {
                if is_player_1 {
                    is_self_turn.0 = true;
                    is_player_1_res.0 = true;
                } else {
                    is_self_turn.0 = false;
                    is_player_1_res.0 = false;
                }
                state.set(GameState::Playing);
            }
            ServerMessage::SpawnCard(card_entity) => {
                let mut sprite = TextureAtlasSprite::new(
                    card_sprites
                        .1
                        .get(&card_entity.get_card().get_name())
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
            ServerMessage::MoveTroop(start_x, start_y, end_x, end_y) => {
                for (_, mut card_entity) in card_entity_q.iter_mut() {
                    if card_entity.get_x_pos() == start_x && card_entity.get_y_pos() == start_y {
                        card_entity.set_x_pos(end_x);
                        card_entity.set_y_pos(end_y);
                        card_entity.moved();
                    }
                }
            }
            ServerMessage::AttackTroop(start_x, start_y, end_x, end_y) => {
                let mut attacker: Option<(Entity, Mut<CardEntity>)> = None;
                let mut attacked: Option<(Entity, Mut<CardEntity>)> = None;
                for (entity, mut card_entity) in card_entity_q.iter_mut() {
                    if card_entity.clone().get_x_pos() == start_x
                        && card_entity.clone().get_y_pos() == start_y
                    {
                        attacker = Some((entity, card_entity));
                    } else if card_entity.clone().get_x_pos() == end_x
                        && card_entity.clone().get_y_pos() == end_y
                    {
                        attacked = Some((entity, card_entity));
                    }
                }
                let mut attacker = attacker.unwrap();
                let mut attacked = attacked.unwrap();
                attacked.1.current_hp -= attacker.1.get_card().get_damage();
                if attacked.1.current_hp <= 0. {
                    commands.entity(attacked.0).despawn();
                    attacked.1.set_x_pos(end_x);
                    attacked.1.set_y_pos(end_y);
                }
            }
            ServerMessage::StartTurn => {
                is_self_turn.0 = true;
            }
            _ => {}
        }
    }
}
