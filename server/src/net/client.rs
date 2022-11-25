use std::collections::VecDeque;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use common::messages::ClientMessage;

use crate::net::threads;

pub struct Client {
    tcp_stream: Arc<Mutex<TcpStream>>,
    packet_queue: Arc<Mutex<VecDeque<ClientMessage>>>,
}

impl Client {
    pub fn new(tcp_stream: TcpStream) -> Client {
        let queue: Arc<Mutex<VecDeque<ClientMessage>>> = Arc::new(Mutex::new(VecDeque::new()));
        threads::spawn(
            Arc::clone(&queue),
            tcp_stream.try_clone().expect("Couldn't clone TcpStream"),
        );
        Client {
            tcp_stream: Arc::new(Mutex::new(tcp_stream)),
            packet_queue: queue,
        }
    }

    pub fn get_stream(&mut self) -> Arc<Mutex<TcpStream>> {
        Arc::clone(&self.tcp_stream)
    }

    pub fn get_packet_queue(&mut self) -> Arc<Mutex<VecDeque<ClientMessage>>> {
        Arc::clone(&self.packet_queue)
    }
}
