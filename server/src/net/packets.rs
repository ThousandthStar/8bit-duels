use std::{
    io::{self, Write},
    net::TcpStream,
};

use serde_json::{ser::to_string, Value};

use crate::game::card::{Card, CardEntity};

pub enum PacketType {
    ServerStartGame,
}

pub struct Packet {
    data: Value,
}

impl Packet {
    pub fn new(type_: PacketType, player_1: bool) -> Packet {
        match type_ {
            PacketType::ServerStartGame => Packet {
                data: serde_json::from_str(if player_1 {
                    r#"
                        {
                            "packet-type":"server-start-game",
                            "player-1":true
                        }
                        "#
                } else {
                    r#"
                        {
                            "packet-type":"server-start-game",
                            "player-1":true
                        }
                        "#
                })
                .unwrap(),
            },
            _ => Packet { data: Value::Null },
        }
    }

    /* `is_owned_by_p1` must be active when we are sending this packet to the second player.
    This flips the coordinates to correctly place the troops on the board
    */
    pub fn spawn_troop(card: &Card, x_pos: i32, y_pos: i32, is_owned_by_p1: bool) -> Packet {
        let mut json: Value = serde_json::from_str(
            r#"
            {
                "packet-type": "spawn-card"        
            }
        "#,
        )
        .unwrap();
        if let Value::Object(ref mut map) = json {
            let card_entity = CardEntity::new(
                card,
                if is_owned_by_p1 { 4 - x_pos } else { x_pos },
                if is_owned_by_p1 { 8 - y_pos } else { y_pos },
                is_owned_by_p1,
            );
            map.insert(
                "troop".to_string(),
                serde_json::to_value(&card_entity).unwrap(),
            );
        }
        Packet { data: json }
    }
}

pub trait WritePacket {
    fn write_packet(&mut self, packet: Packet) -> io::Result<usize>;
}

impl WritePacket for TcpStream {
    fn write_packet(&mut self, packet: Packet) -> io::Result<usize> {
        self.write(to_string(&packet.data).unwrap().as_bytes())
    }
}
