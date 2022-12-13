use std::thread;

use serde_json::Value;

use crate::net::client::Client;
use crate::to_p2_x;
use crate::to_p2_y;
use crate::utils::{self, WritePacket};

use common::card::{Card, CardCollection, CardEntity};
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
                    if let Some(message) = guard.pop_front(){
                        match message {
                            ClientMessage::Deck(deck) => {
                                if player_1{
                                    deck_1 = Some(deck);
                                }
                                else{
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
                guard.write_packet(ServerMessage::SpawnCard(CardEntity::new(cards.0.get("skeleton").unwrap(), 4, 3, true)));
                guard.write_packet(ServerMessage::SpawnCard(CardEntity::new(cards.0.get("skeleton").unwrap(), 3, 3, false)));
                guard = out_2.lock().unwrap();
                guard.write_packet(ServerMessage::StartGame(false));
                guard.write_packet(ServerMessage::SpawnCard(CardEntity::new(cards.0.get("skeleton").unwrap(), to_p2_x!(4), to_p2_y!(3), true)));
                guard.write_packet(ServerMessage::SpawnCard(CardEntity::new(cards.0.get("skeleton").unwrap(), to_p2_x!(3), to_p2_y!(3), false)));
                game_board[3][4] = Some(CardEntity::new(cards.0.get("skeleton").unwrap(), 4, 3, true));
                game_board[3][3] = Some(CardEntity::new(cards.0.get("skeleton").unwrap(), 3, 3, false));
                let mut is_player_1_turn = true;

                info!("starting game");
                loop{
                    if is_player_1_turn{
                        let mut queue_guard = queue_1.lock().unwrap();
                        if let Some(message) = queue_guard.pop_front() {
                            match message {
                                ClientMessage::MoveTroop(start_x, start_y, end_x, end_y) => {
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
                                    {
                                        game_board[start_y as usize][start_x as usize] = None;
                                        card_to_move.moved();
                                        game_board[end_y as usize][end_x as usize] = Some(card_to_move);
                                        guard = out_1.lock().unwrap();
                                        let mut guard_2 = out_2.lock().unwrap();
                                        if is_player_1_turn{
                                            guard.write_packet(ServerMessage::MoveTroop(start_x, start_y, end_x, end_y));
                                            guard_2.write_packet(ServerMessage::MoveTroop(4 - start_x, 8 - start_y, 4 - end_x, 8 - end_y));
                                        }
                                        else{
                                            guard.write_packet(ServerMessage::MoveTroop(4 - start_x, 8 - start_y, 4 - end_x, 8 - end_y));
                                            guard_2.write_packet(ServerMessage::MoveTroop(start_x, start_y, end_x, end_y));
                                        }
                                        drop(guard);
                                        drop(guard_2);
                                    }
                                },
                                ClientMessage::AttackTroop(start_x, start_y, end_x, end_y) => {
                                    let card_to_attack = game_board[start_y as usize][start_x as usize].clone();
                                        let where_to_attack = game_board[end_y as usize][end_x as usize].clone();
                                        println!("b");

                                        if card_to_attack.is_none() || where_to_attack.is_none(){
                                            continue;
                                        }
                                        let mut card_to_attack = card_to_attack.unwrap();
                                        let mut where_to_attack = where_to_attack.unwrap();

                                        if
                                            is_player_1_turn == card_to_attack.is_owned_by_p1()
                                            && !card_to_attack.has_attacked()
                                            && where_to_attack.is_owned_by_p1() != is_player_1_turn
                                        {
                                            println!("a");
                                            card_to_attack.attacked();
                                            card_to_attack.moved();
                                            where_to_attack.current_hp -= card_to_attack.get_card().get_damage();
                                            if where_to_attack.current_hp <= 0. {
                                                game_board[start_y as usize][start_x as usize] = None;
                                                game_board[end_y as usize][end_x as usize] = Some(card_to_attack);
                                            }
                                            else{
                                                game_board[start_y as usize][start_x as usize] = Some(card_to_attack);
                                                game_board[end_y as usize][end_x as usize] = Some(where_to_attack);
                                            }
                                            guard = out_1.lock().unwrap();
                                            let mut guard_2 = out_2.lock().unwrap();
                                            if is_player_1_turn{
                                                guard.write_packet(ServerMessage::AttackTroop(start_x, start_y, end_x, end_y));
                                                guard_2.write_packet(ServerMessage::AttackTroop(4 - start_x, 8 - start_y, 4 - end_x, 8 - end_y));
                                            }else{
                                                guard.write_packet(ServerMessage::AttackTroop(4 - start_x, 8 - start_y, 4 - end_x, 8 - end_y));
                                                guard_2.write_packet(ServerMessage::AttackTroop(start_x, start_y, end_x, end_y));
                                            }
                                            drop(guard);
                                            drop(guard_2);
                                        }


                                },
                                ClientMessage::Deck(_) => {},
                                _ => {}
                            }
                        }
                    }
                }



            }),
        );
    }
}
