use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use serde_json::from_value;
use serde_json::Value;

use crate::net::client::Client;
use crate::net::packets::{Packet, PacketType, WritePacket};
use crate::utils;

use self::card::{Card, CardEntity};

use log::{info, warn};

pub mod card;
pub mod player_deck;

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
        let cards = card::CardCollection::init();
        let game_board = self.game_board.clone();
        thread::spawn(
            closure::closure!(move queue_1, move queue_2, move cards, move mut game_board, ||{
                let mut deck_1: Option<Vec<Card>> = None;
                let mut deck_2: Option<Vec<Card>> = None;
                let mut player_1: bool = true;
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
                    if !guard.is_empty(){
                        let json: Value = serde_json::from_str::<Value>(guard.pop_front().unwrap().as_str()).expect("Bad packet from a client");
                        if let Value::String(packet_type) = json["packet-type"].clone(){
                            if packet_type.as_str() == "player-deck"{
                                if let Value::Array(serialized_deck) = json["deck"].clone(){
                                    let mut deck: Vec<Card> = vec![];
                                    for card in serialized_deck{
                                        deck.push(serde_json::from_value::<Card>(card).unwrap());
                                    }
                                    if player_1{
                                        deck_1 = Some(deck);
                                    }
                                    else{
                                        deck_2 = Some(deck);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    drop(guard);
                }
                let mut guard = out_1.lock().unwrap();
                guard.write_packet(Packet::start_game(PacketType::ServerStartGame, true));
                guard.write_packet(Packet::spawn_troop(cards.0.get("skeleton").unwrap(), 4, 3, true, false));
                drop(guard);
                guard = out_2.lock().unwrap();
                guard.write_packet(Packet::start_game(PacketType::ServerStartGame, false));
                guard.write_packet(Packet::spawn_troop(cards.0.get("skeleton").unwrap(), 4, 3, true, true));
                drop(guard);
                game_board[3][4] = Some(CardEntity::new(cards.0.get("skeleton").unwrap(), 4, 3, true));
                let mut is_player_1_turn = true;

                info!("starting game");
                loop{
                    let mut packet = Value::Null;
                    if is_player_1_turn{
                        let mut guard = queue_1.lock().unwrap();
                        if guard.is_empty(){
                            drop(guard);
                            continue;
                        }
                        packet = serde_json::from_str(guard.pop_front().unwrap().as_str()).unwrap_or(Value::Null);
                        drop(guard);
                    }
                        if matches!(packet, Value::Object(_)){
                            if let Value::String(packet_type) = packet["packet-type"].clone(){
                                match packet_type.as_str(){
                                    "move-troop" => {
                                        let positions = utils::get_targeted_action_positions(packet);

                                        if positions.is_none(){
                                            continue;
                                        }

                                        let (start_x, start_y, end_x, end_y) = positions.unwrap();

                                        let card_to_move = game_board[start_y as usize][start_x as usize].clone();
                                        let where_to_move = game_board[end_y as usize][end_x as usize].clone();

                                        if card_to_move.is_some() && where_to_move.is_none(){
                                            game_board[start_y as usize][start_x as usize] = None;
                                            game_board[end_y as usize][end_x as usize] = Some(card_to_move.unwrap());
                                            guard = out_1.lock().unwrap();
                                            let mut guard_2 = out_2.lock().unwrap();
                                            if is_player_1_turn{
                                                guard.write_packet(Packet::move_troop(start_x, start_y, end_x, end_y));
                                                guard_2.write_packet(Packet::move_troop(4. - start_x, 8. - start_y, 4. - end_x, 8. - end_y));
                                            }
                                            else{
                                                guard.write_packet(Packet::move_troop(4. - start_x, 8. - start_y, 4. - end_x, 8. - end_y));
                                                guard.write_packet(Packet::move_troop(start_x, start_y, end_x, end_y));
                                            }
                                            drop(guard);
                                            drop(guard_2);
                                        }
                                    },
                                    _ => continue,
                                }
                            }

                    }
                }
            }),
        );
    }
}
