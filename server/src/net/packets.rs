use std::{
    io::{self, Write},
    net::TcpStream,
};

use byteorder::{BigEndian, WriteBytesExt};
use serde_json::{ser::to_string, Number, Value};

use crate::game::card::{Card, CardEntity};

pub enum PacketType {
    ServerStartGame,
    StartTurn,
}

pub struct Packet {
    data: Value,
}

impl Packet {
    pub fn start_game(type_: PacketType, player_1: bool) -> Packet {
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
                            "player-1":false
                        }
                        "#
                })
                .unwrap(),
            },
            PacketType::StartTurn => Packet {
                data: serde_json::from_str(
                    r#"
                        {
                            "packet-type":"start-turn"
                        }
                    "#,
                )
                .unwrap(),
            },
            _ => Packet { data: Value::Null },
        }
    }

    /* `is_owned_by_p1` must be active when we are sending this packet to the second player.
    This flips the coordinates to correctly place the troops on the board
    */
    pub fn spawn_troop(
        card: &Card,
        x_pos: i32,
        y_pos: i32,
        is_owned_by_p1: bool,
        flip_board: bool,
    ) -> Packet {
        let mut json = serde_json::from_str(
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
                if flip_board { 4 - x_pos } else { x_pos },
                if flip_board { 8 - y_pos } else { y_pos },
                is_owned_by_p1,
            );
            map.insert(
                "troop".to_string(),
                serde_json::to_value(&card_entity).unwrap(),
            );
        }
        Packet { data: json }
    }

    pub fn move_troop(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Packet {
        let mut json = serde_json::from_str(
            r#"
        {
            "packet-type": "move-troop"
        }
    "#,
        )
        .unwrap();
        if let Value::Object(ref mut map) = json {
            map.insert(
                "start-x".to_string(),
                Value::Number(Number::from_f64(start_x as f64).unwrap()),
            );
            map.insert(
                "start-y".to_string(),
                Value::Number(Number::from_f64(start_y as f64).unwrap()),
            );

            map.insert(
                "end-x".to_string(),
                Value::Number(Number::from_f64(end_x as f64).unwrap()),
            );
            map.insert(
                "end-y".to_string(),
                Value::Number(Number::from_f64(end_y as f64).unwrap()),
            );
        }
        return Packet { data: json };
    }

    pub fn attack_troop(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Packet {
        let mut json = serde_json::from_str(
            r#"
        {
            "packet-type": "move-troop"
        }
    "#,
        )
        .unwrap();
        if let Value::Object(ref mut map) = json {
            map.insert(
                "start-x".to_string(),
                Value::Number(Number::from_f64(start_x as f64).unwrap()),
            );
            map.insert(
                "start-y".to_string(),
                Value::Number(Number::from_f64(start_y as f64).unwrap()),
            );

            map.insert(
                "end-x".to_string(),
                Value::Number(Number::from_f64(end_x as f64).unwrap()),
            );
            map.insert(
                "end-y".to_string(),
                Value::Number(Number::from_f64(end_y as f64).unwrap()),
            );
        }
        return Packet { data: json };
    }
}

pub trait WritePacket {
    fn write_packet(&mut self, packet: Packet) -> io::Result<usize>;
}

impl WritePacket for TcpStream {
    fn write_packet(&mut self, packet: Packet) -> io::Result<usize> {
        let binding = to_string(&packet.data).unwrap();
        let bytes = binding.as_bytes();
        let mut wtr_buf_len = vec![];
        wtr_buf_len
            .write_u32::<BigEndian>(bytes.len() as u32)
            .unwrap();
        self.write(&wtr_buf_len);
        self.write(bytes)
    }
}
