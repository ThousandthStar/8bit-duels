use std::{
    collections::VecDeque,
    io::Write,
    net::TcpStream,
    panic,
    sync::{Arc, Mutex},
    thread,
};

use byteorder::{BigEndian, WriteBytesExt};

pub(crate) fn spawn_output_thread(
    queue_out_ref: Arc<Mutex<VecDeque<String>>>,
    mut stream: TcpStream,
) {
    thread::spawn(move || loop {
        let mut guard = queue_out_ref.lock().unwrap();
        if guard.is_empty() {
            drop(guard);
            continue;
        }
        let binding = guard.pop_front().unwrap();
        let bytes = binding.as_bytes();
        let mut wtr = vec![];
        wtr.write_u32::<BigEndian>(bytes.len() as u32).unwrap();
        stream.write(wtr.as_slice());
        stream.write(bytes);
    });
}
