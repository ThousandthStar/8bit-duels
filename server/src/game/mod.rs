use std::thread;

use crate::net::client::Client;
use crate::to_p2_x;
use crate::to_p2_y;
use crate::utils::{self, WritePacket};
use rustrict::CensorStr;

use common::card::{Card, CardAbility, CardCollection, CardEntity};
use common::messages::{ClientMessage, ServerMessage};

use log::{info, warn};

pub struct Game {
    client_1: Client,
    client_2: Client,
    game_board: [[Option<CardEntity>; 5]; 9],
}

impl Game {
    pub fn new(client_1: Client, client_2: Client) -> Game {
        Game {
            client_1,
            client_2,
            game_board: [
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
            ],
        }
    }

    pub fn run(&mut self) {
        let queue_1 = self.client_1.get_packet_queue();
        let queue_2 = self.client_2.get_packet_queue();
        let out_1 = self.client_1.get_stream();
        let out_2 = self.client_2.get_stream();
        let cards = CardCollection::new();
        let game_board = self.game_board.clone();
        let mut p1_username = "".to_owned();
        let mut p2_username = "".to_owned();
        thread::spawn(
            closure::closure!(move queue_1, move queue_2, move cards, move mut game_board, ||{
                let mut deck_1: Option<Vec<Card>> = None;
                let mut deck_2: Option<Vec<Card>> = None;
                let mut player_1: bool = true;
                let mut player_1_pawns: i32 = 6;
                let mut player_2_pawns: i32 = 6;
                let mut player_1_spirits: i32 = 8;
                let mut player_2_spirits: i32 = 8;
                // first check to get player decks
                loop{
                    let mut guard;
                    if deck_1.is_some(){
                        guard = queue_2.lock().unwrap();
                        player_1 = false;
                    }else{
                        guard = queue_1.lock().unwrap();
                        player_1 = true;
                    }
                    if let Some(message) = guard.pop_front(){
                        match message {
                            ClientMessage::PlayerInfo(username, deck) => {
                                if player_1{
                                    p1_username = username;
                                    deck_1 = Some(deck);
                                }
                                else{
                                    p2_username = username;
                                    deck_2 = Some(deck);
                                    break;
                                }
                            }
                            ClientMessage::MoveTroop(_,_,_,_) => {}
                            ClientMessage::AttackTroop(_,_,_,_) => {}
                            _ => {}

                        }
                    }
                }
                let mut guard = out_1.lock().unwrap();
                guard.write_packet(ServerMessage::StartGame(true));
                guard.write_packet(ServerMessage::StartTurn);
                guard = out_2.lock().unwrap();
                guard.write_packet(ServerMessage::StartGame(false));
                drop(guard);
                let mut is_player_1_turn = true;

                info!("starting game");
                'game_loop: loop{
                    let mut queue_guard;
                    let mut queue_guard_2;
                    if is_player_1_turn{
                        queue_guard = queue_1.lock().unwrap();
                        queue_guard_2 = queue_2.lock().unwrap();
                    }
                    else{
                        queue_guard = queue_2.lock().unwrap();
                        queue_guard_2 = queue_1.lock().unwrap();
                    }
                    let mut index_list: Vec<usize> = Vec::new();
                    for (index, packet) in queue_guard_2.iter().enumerate() {
                        if let ClientMessage::ChatMessage(message) = packet {
                            if message.len() > 30{
                                continue;
                            }
                            let message = &message.censor();
                            let final_message = if is_player_1_turn { p2_username.clone() } else { p1_username.clone() } + ": " + &message;
                            out_1.lock().unwrap().write_packet(ServerMessage::ChatMessage(final_message.clone()));
                            out_2.lock().unwrap().write_packet(ServerMessage::ChatMessage(final_message));
                            index_list.push(index);
                        }
                    }
                    for index in &index_list {
                        queue_guard_2.remove(*index);
                    }
                    drop(queue_guard_2);
                        if let Some(message) = queue_guard.pop_front() {
                            if is_player_1_turn {
                                player_1_spirits += 1;
                            }else{
                                player_2_spirits += 1;
                            }
                            drop(queue_guard);
                            match message {
                                ClientMessage::MoveTroop(mut start_x, mut start_y, mut end_x, mut end_y) => {
                                    if !is_player_1_turn{
                                       start_x = 4 - start_x;
                                       start_y = 8 - start_y;
                                       end_x = 4 - end_x;
                                       end_y = 8 - end_y;
                                    }
                                    let card_to_move = game_board[start_y as usize][start_x as usize].clone();
                                    let where_to_move = game_board[end_y as usize][end_x as usize].clone();
                                    if card_to_move.is_none(){
                                        continue;
                                    }
                                    let mut card_to_move = card_to_move.unwrap();

                                    if
                                        where_to_move.is_none()
                                        && is_player_1_turn == card_to_move.is_owned_by_p1()
                                        && !card_to_move.has_attacked()
                                        && !card_to_move.has_moved()
                                        && card_to_move.stun_count == 0
                                    {
                                        game_board[start_y as usize][start_x as usize] = None;
                                        card_to_move.moved();
                                        game_board[end_y as usize][end_x as usize] = Some(card_to_move);
                                        guard = out_1.lock().unwrap();
                                        let mut guard_2 = out_2.lock().unwrap();
                                        guard.write_packet(ServerMessage::MoveTroop(start_x, start_y, end_x, end_y));
                                        guard_2.write_packet(ServerMessage::MoveTroop(4 - start_x, 8 - start_y, 4 - end_x, 8 - end_y));
                                        drop(guard);
                                        drop(guard_2);
                                    }
                                },
                                ClientMessage::AttackTroop(mut start_x, mut start_y, mut end_x, mut end_y) => {
                                    if !is_player_1_turn{
                                       start_x = 4 - start_x;
                                       start_y = 8 - start_y;
                                       end_x = 4 - end_x;
                                       end_y = 8 - end_y;
                                    }
                                    let card_to_attack = game_board[start_y as usize][start_x as usize].clone();
                                        let where_to_attack = game_board[end_y as usize][end_x as usize].clone();

                                        if card_to_attack.is_none() || where_to_attack.is_none(){
                                            continue;
                                        }
                                        let mut card_to_attack = card_to_attack.unwrap();
                                        let mut where_to_attack = where_to_attack.unwrap();

                                        if
                                            is_player_1_turn == card_to_attack.is_owned_by_p1()
                                            && !card_to_attack.has_attacked()
                                            && where_to_attack.is_owned_by_p1() != is_player_1_turn
                                            && card_to_attack.stun_count == 0
                                        {
                                            card_to_attack.moved();
                                            let card_binding = card_to_attack.get_card();
                                            let abilities = card_binding.get_abilities();
                                            for ability in &abilities {
                                                if let CardAbility::Stun { amount } = ability {
                                                    where_to_attack.stun_count += amount;
                                                }
                                            }
                                            card_to_attack.attacked();

                                            where_to_attack.current_hp -= card_to_attack.get_card().get_damage();
                                            if where_to_attack.current_hp <= 0. {
                                                game_board[start_y as usize][start_x as usize] = None;
                                                game_board[end_y as usize][end_x as usize] = Some(card_to_attack.clone());
                                                let spirits_to_add = if abilities.contains(&CardAbility::SpiritCollector) {
                                                    where_to_attack.get_card().get_cost()
                                                } else {
                                                    where_to_attack.get_card().get_cost() / 2
                                                };
                                                if card_to_attack.is_owned_by_p1() {
                                                    player_1_spirits += spirits_to_add;
                                                    player_2_pawns += 1;
                                                } else{
                                                    player_2_spirits += spirits_to_add;
                                                    player_1_pawns += 1;
                                                }
                                            }
                                            else{
                                                game_board[start_y as usize][start_x as usize] = Some(card_to_attack);
                                                game_board[end_y as usize][end_x as usize] = Some(where_to_attack);
                                            }
                                            guard = out_1.lock().unwrap();
                                            let mut guard_2 = out_2.lock().unwrap();
                                            guard.write_packet(ServerMessage::AttackTroop(start_x, start_y, end_x, end_y));
                                            guard_2.write_packet(ServerMessage::AttackTroop(4 - start_x, 8 - start_y, 4 - end_x, 8 - end_y));
                                            drop(guard);
                                            drop(guard_2);
                                        }
                                },
                                ClientMessage::EndTurn => {
                                    if is_player_1_turn {
                                        out_2.lock().unwrap().write_packet(ServerMessage::StartTurn);
                                    } else{
                                        out_1.lock().unwrap().write_packet(ServerMessage::StartTurn);
                                    }
                                    is_player_1_turn = !is_player_1_turn;
                                    for mut arr in &mut game_board {
                                       for mut option in arr{
                                           if option.is_some() {
                                               option.as_mut().unwrap().reset();
                                           }
                                       }
                                    }
                                },
                                ClientMessage::SpawnCard(card, mut x, mut y) => {
                                    if x > 4 || x < 0 || y > 8 || y < 0 {return;}
                                    if is_player_1_turn{
                                        if player_1_pawns < 1 || player_1_spirits < card.get_cost() {
                                            continue;
                                        }
                                    }
                                    else{
                                        if player_2_pawns < 1 || player_2_spirits < card.get_cost() {
                                            continue;
                                        }
                                    }

                                    if let Some(_) = game_board[y as usize][x as usize] {
                                        continue;
                                    }

                                    let mut card_entity = CardEntity::new(&card, x, y, is_player_1_turn);
                                    game_board[y as usize][x as usize] = Some(card_entity.clone());
                                    if !is_player_1_turn {
                                        player_2_pawns -= 1;
                                        player_2_spirits -= card.get_cost();
                                    }
                                    else{
                                        player_1_pawns -= 1;
                                        player_1_spirits -= card.get_cost();
                                    }
                                    out_1.lock().unwrap().write_packet(ServerMessage::SpawnCard(card_entity.clone()));
                                    card_entity.set_x_pos(to_p2_x!(x));
                                    card_entity.set_y_pos(to_p2_y!(y));
                                    out_2.lock().unwrap().write_packet(ServerMessage::SpawnCard(card_entity));
                                },
                                ClientMessage::WinGame(x, y) => {
                                    if let Some(card_entity) = &game_board[y as usize][x as usize] {
                                        if y != 0 && y != 8{
                                            continue;
                                        }
                                        if card_entity.is_owned_by_p1() == is_player_1_turn{
                                            if is_player_1_turn && !(card_entity.stun_count > 0) && !card_entity.has_moved(){
                                                out_1.lock().unwrap().write_packet(ServerMessage::EndGame(true));
                                                out_2.lock().unwrap().write_packet(ServerMessage::EndGame(false));
                                                break 'game_loop;
                                            }
                                            else if !(card_entity.stun_count > 0) && !card_entity.has_moved(){
                                                out_1.lock().unwrap().write_packet(ServerMessage::EndGame(false));
                                                out_2.lock().unwrap().write_packet(ServerMessage::EndGame(true));
                                                break 'game_loop;
                                            }
                                        }
                                    }
                                }
                                ClientMessage::ChatMessage(message) => {
                                    if message.len() > 30{
                                        continue;
                                    }
                                    let message = &message.censor();
                                    let final_message = if is_player_1_turn { p2_username.clone() } else { p1_username.clone() } + ": " + &message;
                                    out_1.lock().unwrap().write_packet(ServerMessage::ChatMessage(final_message.clone()));
                                    out_2.lock().unwrap().write_packet(ServerMessage::ChatMessage(final_message));
                                }
                                _ => {}
                            }
                        }
                    }
            }),
        );
    }
}
