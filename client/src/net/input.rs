use std::{
    collections::VecDeque,
    io::{BufReader, Read},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

pub(crate) fn spawn_input_thread(
    queue_in_ref: Arc<Mutex<VecDeque<String>>>,
    mut reader: BufReader<TcpStream>,
) {
    thread::spawn(move || {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut received_data: usize;
        loop {
            received_data = reader.read(&mut buffer).unwrap_or(0);
            if received_data > 0 {
                if let Ok(string) = std::str::from_utf8_mut(&mut buffer) {
                    let mut guard = queue_in_ref.lock().unwrap();
                    guard.push_back(string.replace("\0", "").to_string());
                    drop(guard);
                    buffer = [0; 1024];
                } else {
                    println!("Got an invalid UTF-8 packet!")
                }
            } else {
                break;
            }
        }
    });
}
