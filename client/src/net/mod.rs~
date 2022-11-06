mod input;
mod out;
pub(crate) mod packet_handler;

use std::{
    collections::VecDeque,
    io::BufReader,
    net::TcpStream,
    result,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;

pub(crate) struct QueueIn(pub(crate) Arc<Mutex<VecDeque<String>>>);
pub(crate) struct QueueOut(pub(crate) Arc<Mutex<VecDeque<String>>>);

pub(crate) fn init(commands: &mut Commands) {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:1000") {
        let queue_in: VecDeque<String> = VecDeque::new();
        let queue_out: VecDeque<String> = VecDeque::new();

        let queue_in_arc = Arc::new(Mutex::new(queue_in));
        let queue_out_arc = Arc::new(Mutex::new(queue_out));

        if let Ok(cloned_stream) = stream.try_clone() {
            input::spawn_input_thread(Arc::clone(&queue_in_arc), BufReader::new(cloned_stream));
            out::spawn_output_thread(Arc::clone(&queue_out_arc), stream);

            commands.insert_resource(QueueIn(queue_in_arc));
            commands.insert_resource(QueueOut(queue_out_arc));
        } else {
            panic!("Could not clone TCP stream")
        }
    } else {
        panic!("Could not connect to the server!");
    }
}
