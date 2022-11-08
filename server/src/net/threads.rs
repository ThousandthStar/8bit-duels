use std::collections::VecDeque;
use std::io::{BufReader, Cursor, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

use byteorder::{BigEndian, ReadBytesExt};

pub fn spawn(queue: Arc<Mutex<VecDeque<String>>>, mut stream: TcpStream) {
    thread::spawn(closure::closure!(move queue, || {
        let mut received_data: usize;
        let mut cursor: Cursor<[u8; 4]>;
        loop {
            let mut buffer_size_read: [u8; 4] = [0; 4];
            stream.read(&mut buffer_size_read);
            cursor = Cursor::new(buffer_size_read);
            let mut buffer = vec![0; cursor.read_u32::<BigEndian>().unwrap() as usize];
            received_data = stream.read(&mut buffer).expect("Couldn't read packet");
            if received_data > 0 {
                let string = std::str::from_utf8_mut(&mut buffer).expect("Invalid UTF-8 packet!");
                let mut guard = queue.lock().unwrap();
                guard.push_back(string.to_string().trim().replace("\0", "").to_string());
                drop(guard);
            } else {
                break;
            }
        }
    }));
}
