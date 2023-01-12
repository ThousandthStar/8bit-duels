use super::{QueueIn, QueueOut};
use crate::{
    card_interactions::is_in_boundary,
    currency::{Pawns, Spirits},
    tilemap::{CardSprites, TileSize},
    ui::in_game_ui::ChatMessages,
    GameState, IsPlayer1, IsSelfTurn,
};
use bevy::prelude::*;
use common::{
    card::{CardAbility, CardEntity},
    messages::ServerMessage,
};

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
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    card_sprites: Res<CardSprites>,
    tile_size: Res<TileSize>,
    mut card_entity_q: Query<(Entity, &mut CardEntity)>,
    mut visible_q: Query<&mut Visibility, Without<CardEntity>>,
    mut is_self_turn: ResMut<IsSelfTurn>,
    mut is_player_1_res: ResMut<IsPlayer1>,
    mut pawn_count: ResMut<Pawns>,
    mut spirit_count: ResMut<Spirits>,
    mut chat_messages: ResMut<ChatMessages>,
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
                    .spawn(SpriteSheetBundle {
                        sprite,
                        texture_atlas: card_sprites.0.clone(),
                        transform: Transform::from_xyz(1000000000.0, 1000000000.0, 500.),
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
                attacker.1.attacked();
                let abilities = attacker.1.get_card().get_abilities();
                for ability in &abilities {
                    if let CardAbility::Stun { amount } = ability {
                        attacked.1.stun_count += amount;
                    }
                }
                if attacked.1.current_hp <= 0. {
                    commands.entity(attacked.0).despawn();
                    attacker.1.set_x_pos(end_x);
                    attacker.1.set_y_pos(end_y);
                    if attacked.1.is_owned_by_p1() == is_player_1_res.0 {
                        pawn_count.0 += 1;
                    }
                    if attacker.1.is_owned_by_p1() == is_player_1_res.0 {
                        if abilities.contains(&CardAbility::SpiritCollector) {
                            spirit_count.0 += attacked.1.get_card().get_cost();
                        } else {
                            spirit_count.0 += attacked.1.get_card().get_cost() / 2;
                        }
                    }
                }
            }
            ServerMessage::StartTurn => {
                is_self_turn.0 = true;
                for (_, mut card_entity) in card_entity_q.iter_mut() {
                    if card_entity.is_owned_by_p1() == is_player_1_res.0 {
                        card_entity.reset();
                    }
                }
                spirit_count.0 += 1;
            }
            ServerMessage::EndGame(won) => {
                for (entity, _) in card_entity_q.iter() {
                    commands.entity(entity).despawn();
                }
                for mut visibility in visible_q.iter_mut() {
                    visibility.is_visible = false;
                }
                state.set(GameState::Waiting);
            }
            ServerMessage::ChatMessage(message) => {
                chat_messages.0.push(message);
            }
            _ => {}
        }
    }
}
