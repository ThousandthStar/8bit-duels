use byteorder::{BigEndian, WriteBytesExt};
use common::messages::ServerMessage;
use serde_json::Value;
use std::error::Error;
use std::io::{self, Write};
use std::net::TcpStream;

#[macro_export]
macro_rules! to_p2_x {
    ($x:expr) => {
        4 - $x
    };
}

#[macro_export]
macro_rules! to_p2_y {
    ($y: expr) => {
        8 - $y
    };
}

pub(crate) struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub(crate) fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub(crate) fn distance(&self, other: Vec2) -> i32 {
        (((self.x as f32) - (other.x as f32)).powf(2.)
            + ((self.y as f32) - (other.y as f32)).powf(2.))
        .sqrt() as i32
    }
}

pub trait WritePacket {
    fn write_packet(&mut self, packet: ServerMessage);
}

impl WritePacket for TcpStream {
    fn write_packet(&mut self, packet: ServerMessage) {
        if let Ok(binding) = serde_json::to_string(&packet) {
            let bytes = binding.as_bytes();
            let mut wtr_buf_len = vec![];
            wtr_buf_len
                .write_u32::<BigEndian>(bytes.len() as u32)
                .unwrap();
            self.write(wtr_buf_len.as_slice());
            self.write(bytes);
        }
    }
}
