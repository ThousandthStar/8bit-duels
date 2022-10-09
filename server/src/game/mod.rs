use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use serde_json::from_value;
use serde_json::Value;

use crate::net::client::Client;
use crate::net::packets::{Packet, PacketType, WritePacket};

use self::card::{Card, CardEntity};

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
        let game_board = Arc::new(Mutex::new(&self.game_board));
        let cards = card::CardCollection::init();
        thread::spawn(
            closure::closure!(move queue_1, move queue_2, move cards, ||{
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
                guard.write_packet(Packet::new(PacketType::ServerStartGame, true));
                guard.write_packet(Packet::spawn_troop(cards.0.get("skeleton").unwrap(), 4, 3, false));
                drop(guard);
                guard = out_2.lock().unwrap();
                guard.write_packet(Packet::new(PacketType::ServerStartGame, false));
                guard.write_packet(Packet::spawn_troop(cards.0.get("skeleton").unwrap(), 4, 3, true));
                drop(guard);
                println!("Game Started");
                loop{

                }
            }),
        );
    }
}
