mod game;
mod net;
mod utils;

use std::{
    fmt::format,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use common::card::CardCollection;
use game::Game;
use log::{info, warn};
use net::client::Client;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    const port: i32 = 1000;
    let card_collection = CardCollection::new();
    let listener: TcpListener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Couldn't bind port");
    let mut pending: Option<Client> = None;
    info!("server started");
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                info!("client connected");
                if pending.is_none() {
                    pending = Some(Client::new(stream));
                } else {
                    info!("starting game instance");
                    Game::new(pending.unwrap(), Client::new(stream)).run();
                    pending = None;
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn card_deserialization_test() {
        let card_collection = CardCollection::init();
        // run `cargo test -- --nocapture` to see output
        println!(
            "{}",
            serde_json::to_string(&card_collection.0.get(&String::from("skeleton"))).unwrap()
        );
    }
}
