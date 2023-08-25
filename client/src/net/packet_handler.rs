use super::QueueIn;
use crate::{
    animations::AttackAnimation,
    card_interactions::{SelectIndicator, ViewingCardEntity},
    currency::{Pawns, Spirits},
    ownership_indicator::OwnershipIndicator,
    tilemap::{self, CardSprites, TileSize},
    ui::{
        in_game_ui::{EndTurnButtonLabel, TurnIndicator},
        GameFont,
    },
    GameState, IsPlayer1, IsSelfTurn,
};
use belly::prelude::*;
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
        .insert_resource(ChatMessages(Vec::new()))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_packets));
    }
}

#[derive(Resource, Clone)]
pub struct ChatMessages(pub Vec<String>);

fn handle_packets(
    queue_in: ResMut<QueueIn>,
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    card_sprites: Res<CardSprites>,
    tile_size: Res<TileSize>,
    mut card_entity_q: Query<(Entity, &mut CardEntity, &Transform)>,
    mut visible_q: Query<
        &mut Visibility,
        (
            Without<CardEntity>,
            Without<Label>,
            Without<SelectIndicator>,
        ),
    >,
    mut is_self_turn: ResMut<IsSelfTurn>,
    mut is_player_1_res: ResMut<IsPlayer1>,
    mut pawn_count: ResMut<Pawns>,
    mut spirit_count: ResMut<Spirits>,
    mut turn_label_q: Query<&mut Label, (With<TurnIndicator>, Without<CardEntity>)>,
    game_font: Res<GameFont>,
    asset_server: Res<AssetServer>,
    mut elements: Elements,
    mut messages: ResMut<ChatMessages>,
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
                state.set(GameState::Playing).unwrap();
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
                let is_owned_by_p1 = card_entity.is_owned_by_p1();

                commands
                    .spawn(SpriteSheetBundle {
                        sprite,
                        texture_atlas: card_sprites.0.clone(),
                        transform: Transform::from_xyz(1000000000.0, 1000000000.0, 500.),
                        ..Default::default()
                    })
                    .insert(card_entity)
                    .insert(tilemap::InstantMove)
                    .with_children(move |parent| {
                        let mut transform = Transform::default();
                        transform.translation.z = 400.0;
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(tile_size.0 * 0.9)),
                                    color: Color::hex(if is_owned_by_p1 == is_player_1_res.0 {
                                        "2b8fc4"
                                    } else {
                                        "e0828a"
                                    })
                                    .unwrap(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(OwnershipIndicator);
                    });
            }
            ServerMessage::MoveTroop(start_x, start_y, end_x, end_y) => {
                for (_, mut card_entity, _) in card_entity_q.iter_mut() {
                    if card_entity.get_x_pos() == start_x && card_entity.get_y_pos() == start_y {
                        card_entity.set_x_pos(end_x);
                        card_entity.set_y_pos(end_y);
                        card_entity.moved();
                    }
                }
            }
            ServerMessage::AttackTroop(start_x, start_y, end_x, end_y) => {
                let mut attacker: Option<(Entity, Mut<CardEntity>, &Transform)> = None;
                let mut attacked: Option<(Entity, Mut<CardEntity>, &Transform)> = None;
                for (entity, card_entity, transform) in card_entity_q.iter_mut() {
                    if card_entity.clone().get_x_pos() == start_x
                        && card_entity.clone().get_y_pos() == start_y
                    {
                        attacker = Some((entity, card_entity, transform));
                    } else if card_entity.clone().get_x_pos() == end_x
                        && card_entity.clone().get_y_pos() == end_y
                    {
                        attacked = Some((entity, card_entity, transform));
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
                commands.entity(attacker.0).insert(AttackAnimation {
                    target: Vec2::new(attacked.2.translation.x, attacked.2.translation.y),
                    initial: Vec2::new(attacker.2.translation.x, attacker.2.translation.y),
                    moving_back: false,
                });
                if attacked.1.current_hp <= 0. {
                    commands.entity(attacked.0).despawn_recursive();
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
                for (_, mut card_entity, _) in card_entity_q.iter_mut() {
                    if card_entity.is_owned_by_p1() == is_player_1_res.0 {
                        card_entity.reset();
                    }
                }
                spirit_count.0 += 1;
                if is_self_turn.0 {
                    let button_handle: Handle<Image> = asset_server.load("button.png");
                    let tile_size = tile_size.0;
                    elements.select("#left-panel").add_child(eml! {
                        <button
                            id="end-turn-button"
                            s:width=format!("{}px", tile_size * 3.6)
                            s:height=format!("{}px", tile_size * 0.9)
                        >
                            <img src=button_handle mode="fit" id="end-turn-button-img">
                                <label
                                    with=EndTurnButtonLabel
                                    s:font-size=format!("{}", tile_size / 4.0)
                                    id="end-turn-button-span"
                                    value="End Turn"
                                >
                                </label>
                            </img>
                        </button>

                    });
                }
                turn_label_q.single_mut().value = "Your Turn".to_string();
            }
            ServerMessage::EndGame(_won) => {
                for (entity, _, _) in card_entity_q.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                for mut visibility in visible_q.iter_mut() {
                    visibility.is_visible = false;
                }
                commands.add(|world: &mut World| {
                    world
                        .query_filtered::<&mut Visibility, With<SelectIndicator>>()
                        .single_mut(world)
                        .is_visible = false;
                    world.get_resource_mut::<ViewingCardEntity>().unwrap().0 = None;
                });
                elements.select("body").remove();
                state.set(GameState::Waiting).unwrap();
            }
            ServerMessage::ChatMessage(message) => {
                messages.0.push(message.clone());
                let tile_size = tile_size.0;
                let mut entities = elements.select(".chat-message").entities();
                while entities.len() > 8 {
                    commands.entity(entities.remove(0)).despawn_recursive();
                }
                elements.select("#chat-area").add_child(eml! {
                    <label
                        s:color="black"
                        value=message.clone().as_str()
                        s:font-size=format!("{}", tile_size / 4.0)
                        s:top="0px"
                        s:width="90%"
                        s:left="0%"
                        c:chat-message
                        >
                    </label>
                });
            }
        }
    }
}
