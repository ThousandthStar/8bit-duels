mod input;
mod out;
pub(crate) mod packet_handler;

use std::{
    collections::VecDeque,
    error::Error,
    io::{self, BufReader},
    net::TcpStream,
    result,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use common::messages::{ClientMessage, ServerMessage};

#[derive(Resource)]
pub(crate) struct QueueIn(pub(crate) Arc<Mutex<VecDeque<ServerMessage>>>);
#[derive(Resource)]
pub(crate) struct QueueOut(pub(crate) Arc<Mutex<VecDeque<ClientMessage>>>);

pub(crate) fn init(commands: &mut Commands, server_address: &str) -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect(server_address)?;
    let queue_in: VecDeque<ServerMessage> = VecDeque::new();
    let queue_out: VecDeque<ClientMessage> = VecDeque::new();

    let queue_in_arc = Arc::new(Mutex::new(queue_in));
    let queue_out_arc = Arc::new(Mutex::new(queue_out));

    let cloned_stream = stream.try_clone()?;
    input::spawn_input_thread(Arc::clone(&queue_in_arc), BufReader::new(cloned_stream));
    out::spawn_output_thread(Arc::clone(&queue_out_arc), stream);

    commands.insert_resource(QueueIn(queue_in_arc));
    commands.insert_resource(QueueOut(queue_out_arc));
    bevy::log::info!("Successfully established TCP connection");
    Ok(())
}
