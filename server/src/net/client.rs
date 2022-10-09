use std::collections::VecDeque;
use std::net::TcpStream;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use crate::net::threads;

pub struct Client{
    tcp_stream: Arc<Mutex<TcpStream>>,
    packet_queue: Arc<Mutex<VecDeque<String>>>,
}

impl Client {
    pub fn new(tcp_stream: TcpStream) -> Client{
        let reader: BufReader<TcpStream> = BufReader::new(tcp_stream.try_clone().expect("Couldn't clone TcpStream"));
        let queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
        threads::spawn(Arc::clone(&queue), reader);
        Client { 
            tcp_stream: Arc::new(Mutex::new(tcp_stream)),
            packet_queue: queue,
        }
    }

    pub fn get_stream(&mut self) -> Arc<Mutex<TcpStream>>{
        Arc::clone(&self.tcp_stream)
    }

    pub fn get_packet_queue(&mut self) -> Arc<Mutex<VecDeque<String>>>{
        Arc::clone(&self.packet_queue)
    }
}