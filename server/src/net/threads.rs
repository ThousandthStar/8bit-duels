use std::collections::VecDeque;
use std::io::{BufReader, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn(queue: Arc<Mutex<VecDeque<String>>>, mut reader: BufReader<TcpStream>) {
    thread::spawn(closure::closure!(move queue, || {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut received_data: usize;
        loop {
            received_data = reader.read(&mut buffer).expect("Couldn't read packet");
            if received_data > 0 {
                let string = std::str::from_utf8_mut(&mut buffer).expect("Invalid UTF-8 packet!");
                let mut guard = queue.lock().unwrap();
                guard.push_back(string.to_string().trim().replace("\0", "").to_string());
                drop(guard);
                buffer = [0; 1024];
            } else {
                break;
            }
        }
    }));
}
