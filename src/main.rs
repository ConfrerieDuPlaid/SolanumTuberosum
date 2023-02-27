use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

use rand::Rng;

use md5hash_cash::MD5HashCashResolver;
use structs::ChallengeResolve;

use crate::recover_secret::RecoverSecretResolver;
use crate::structs::{Challenge, Message, PublicPlayer};
use crate::structs::ChallengeAnswer;

mod structs;
mod md5hash_cash;
mod recover_secret;

/// Client resolving challenges.
/// It will connect to the server and will receive challenges.
/// It will resolve the challenges and send the answer to the server.


fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            let message = Message::Hello;
            send_message(&stream, message);

            let mut game: Game = Game::create_game();
            game.receive_messages(&stream);
        },
        Err(_) => panic!("Could not connect to server "),
    }

}

pub struct Game {
    public_leader_board: Vec<PublicPlayer>,
}

impl Game {
    fn create_game () -> Game{
        const GAME: Game = Game{ public_leader_board: vec![] };
        return GAME
    }

    /// Receive messages from the server.
    /// It will read the buffer size and then read the buffer.
    /// It will deserialize the buffer and dispatch the message.
    fn receive_messages(&mut self, mut stream: &TcpStream){
        loop {
            let mut buf_size = [0; 4];
            stream.read(&mut buf_size).expect("Unable to read buffer");
            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                continue
            }

            let mut buf = vec![0; res_size as usize];
            stream.read(&mut buf).expect("Unable to read buffer");
            let string_receive = String::from_utf8_lossy(&buf);
            match serde_json::from_str(&string_receive) {
                Ok(message) => self.dispatch_messages(stream, message),
                Err(err) => println!("Error while parsing message = {}", err),
            }
        }
    }

    /// Dispatch the message received from the server.
    /// Match the message and call the corresponding function.
    fn dispatch_messages(&mut self, mut stream: &TcpStream, message: Message) {
        match message {
            Message::Welcome { version: _version } => {
                let mut rng = rand::thread_rng();
                let n1: u8 = rng.gen();

                let answer = Message::Subscribe { name: "Player".to_string() + &n1.to_string() };
                send_message(&stream, answer);
            }
            Message::SubscribeResult(_subscribe_result) => {
                //println!("SubscribeResult")
            }
            Message::PublicLeaderBoard(public_leader_board ) => {
                for player in public_leader_board {
                    self.public_leader_board.push(PublicPlayer{
                        name: player.name,
                        stream_id: "0".to_string(),
                        score: 0,
                        steps: 0,
                        is_active: false,
                        total_used_time: 0.0,
                    });
                }
            }
            Message::ChallengeTimeout(message) => {
                println!("message = {}", message);
            },
            Message::RoundSummary{ challenge: _challenge, chain: _chain } => {
                //println!("_challenge = {} _chain = {:?}", _challenge, _chain);
            }
            Message::Challenge(challenge) => {
                let answer = match challenge {
                    Challenge::MD5HashCash(md5) => {
                        let solver = MD5HashCashResolver::new(md5);
                        ChallengeAnswer::MD5HashCash(solver.solve())
                    },
                    Challenge::RecoverSecret(recover_secret) => {
                        let solver = RecoverSecretResolver::new(recover_secret);
                        ChallengeAnswer::RecoverSecret(solver.solve())
                    }
                };
                let message: Message;
                let next_target: Option<&PublicPlayer> = self.public_leader_board.get(0);
                match next_target {
                    None => {
                        message = Message::ChallengeResult {
                            answer,
                            next_target: "".to_string()
                        };
                        send_message(&stream, message);
                    }
                    Some(target) => {
                        message = Message::ChallengeResult {
                            answer,
                            next_target: target.clone().name
                        };
                        send_message(&stream, message);
                    }
                }
            }
            Message::EndOfGame { leader_board } => {
                println!("leader_board = {:?}", leader_board);
                stream.flush().expect("Unable to flush stream");
                exit(0);
            }
            _ => {print!("Error")}
        }

    }
}


/// Send a message to the server.
/// It will serialize the message and send the buffer size and the buffer.
/// The buffer size is a u32.
/// The buffer size is sent in big endian.
fn send_message(mut stream: &TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let bytes_message = message.as_bytes();
        let message_size = bytes_message.len() as u32;

        let message_length_as_bytes = message_size.to_be_bytes();
        stream.write(&message_length_as_bytes).expect("Unable to send message");

        stream.write(bytes_message).expect("Unable to send message");
    }
}