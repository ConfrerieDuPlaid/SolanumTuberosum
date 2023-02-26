use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

use rand::Rng;
use serde::{Deserialize, Serialize};

use MD5HashCash::MD5HashCashResolver;
use structs::ChallengeResolve;

use crate::RecoverSecret::RecoverSecretResolver;
use crate::structs::{Challenge, MD5HashCashInput, Message, PublicLeaderBoard, PublicPlayer};
use crate::structs::ChallengeAnswer;
use crate::structs::MD5HashCashOutput;

mod structs;
mod MD5HashCash;
mod RecoverSecret;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
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
    fn create_game () -> Client{
        const client: Client = Client{ public_leader_board: vec![] };
        return client
    }

    fn receive_messages(&mut self, mut stream: &TcpStream){
        loop {
            let mut buf_size = [0; 4];
            stream.read(&mut buf_size);
            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                continue
            }

            let mut buf = vec![0; res_size as usize];
            stream.read(&mut buf);
            let string_receive = String::from_utf8_lossy(&buf);
            match serde_json::from_str(&string_receive) {
                Ok(message) => self.dispatch_messages(stream, message),
                Err(err) => println!("Error while parsing message = {}", err),
            }
        }
    }

    fn dispatch_messages(&mut self, mut stream: &TcpStream, message: Message) {
        match message {
            Message::Welcome { version } => {
                let mut rng = rand::thread_rng();
                let n1: u8 = rng.gen();

                let answer = Message::Subscribe { name: "Player".to_string() + &n1.to_string() };
                send_message(&stream, answer);
            }
            Message::SubscribeResult(subscribeResult) => {
                //println!("SubscribeResult")
            }
            Message::PublicLeaderBoard( publicLeaderBoard ) => {
                for player in publicLeaderBoard{
                    self.public_leader_board.push(PublicPlayer{
                        name: player.name,
                        stream_id: "0".to_string(),
                        score: 0,
                        steps: 0,
                        is_active: false,
                        total_used_time: 0.0,
                    });
                }
                println!("publicLeaderBoard = {:?}", self.public_leader_board.get(0).unwrap().name);

            },
            Message::ChallengeTimeout(message) => {
                println!("message = {}", message);
            },
            Message::RoundSummary{ challenge, chain} => {
                //println!("challenge = {} chain = {:?}", challenge, chain);
            }
            Message::Challenge(challenge) => {
                let answer = match challenge {
                    Challenge::MD5HashCash(md5) => {
                        let solver = MD5HashCashResolver::new(md5);
                        ChallengeAnswer::MD5HashCash(solver.solve())
                    },
                    Challenge::RecoverSecret(recoverSecret) => {
                        let solver = RecoverSecretResolver::new(recoverSecret);
                        ChallengeAnswer::RecoverSecret(solver.solve())
                    }
                };
                let mut message: Message;
                let next_target: Option<&PublicPlayer> = self.public_leader_board.get(0);
                match next_target {
                    None => {
                        message = Message::ChallengeResult {
                            answer,
                            next_target: "".to_string()
                        };
                        println!("coucou");
                        send_message(&stream, message);
                    }
                    Some(target) => {
                        println!("Hello");

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
                stream.flush();
                exit(0);
            }
            _ => {print!("Error")}
        }

    }
}



fn send_message(mut stream: &TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let bytes_message = message.as_bytes();
        let message_size = bytes_message.len() as u32;

        let message_length_as_bytes = message_size.to_be_bytes();
        stream.write(&message_length_as_bytes).unwrap();

        stream.write(bytes_message).unwrap();
    }
}