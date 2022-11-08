use std::{
    collections::VecDeque,
    io::{BufReader, Cursor, Read},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use byteorder::{BigEndian, ReadBytesExt};

pub(crate) fn spawn_input_thread(
    queue_in_ref: Arc<Mutex<VecDeque<String>>>,
    mut reader: BufReader<TcpStream>,
) {
    thread::spawn(move || {
        let mut received_data: usize;
        let mut cursor: Cursor<[u8; 4]>;
        loop {
            let mut buffer_size: [u8; 4] = [0; 4];
            reader.read(&mut buffer_size);
            cursor = Cursor::new(buffer_size);
            let mut buffer = vec![0; cursor.read_u32::<BigEndian>().unwrap() as usize];
            received_data = reader.read(&mut buffer).unwrap_or(0);
            if received_data > 0 {
                if let Ok(string) = std::str::from_utf8_mut(&mut buffer) {
                    let mut guard = queue_in_ref.lock().unwrap();
                    guard.push_back(string.replace("\0", "").to_string());
                } else {
                    println!("Got an invalid UTF-8 packet!")
                }
            } else {
                break;
            }
        }
    });
}
