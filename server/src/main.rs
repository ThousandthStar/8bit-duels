mod net;
mod game;

use std::{fmt::format, thread, net::{TcpListener, TcpStream}, io::{BufReader, Write}};

use game::{card::CardCollection, Game};
use net::{client::Client};

fn main() {
    const port: i32 = 1000;
    let card_collection = CardCollection::init();
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Couldn't bind port");
    let mut pending: Option<Client> = None;
    println!("Server started");
    loop{
        match listener.accept(){
            Ok((stream, _)) => {
                println!("Client connected");
                if pending.is_none(){
                    pending = Some(Client::new(stream));
                }else{
                    println!("Starting game");
                    Game::new(pending.unwrap(), Client::new(stream)).run();
                    pending = None;
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn card_deserialization_test(){
        let card_collection = CardCollection::init();
        // run `cargo test -- --nocapture` to see output
        println!("{}", serde_json::to_string(&card_collection.0.get(&String::from("skeleton"))).unwrap());
    }
}
