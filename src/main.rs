extern crate core;
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::structs::Message;
use crate::structs::MD5HashCashOutput;
use crate::structs::ChallengeAnswer;

mod structs;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            let message = Message::Hello;
            send_message(&stream, message);

            receive_messages(&stream);
        },
        Err(_) => panic!("Could not connect to server "),
    }

}

fn receive_messages(mut stream: &TcpStream){
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
            Ok(message) => dispatch_messages(stream, message),
            Err(_) => println!("Error while parsing message"),
        }
    }
}

fn dispatch_messages(mut stream: &TcpStream, message: Message) {
    println!("Dispatching: {:?}", message);
    match message {
        Message::Welcome { version } => {
            let mut rng = rand::thread_rng();
            let n1: u8 = rng.gen();

            let answer = Message::Subscribe { name: "Player".to_string() + &n1.to_string() };
            send_message(&stream, answer);
        }
        Message::SubscribeResult(subscribeResult) => {
            println!("SubscribeResult")
        }
        Message::PublicLeaderBoard( publicLeaderBoard ) => {
            println!("{:?}", publicLeaderBoard);
        },
        Message::Challenge(challenge) => {
            println!("{:?}", challenge);
            let result = Message::ChallengeResult {
                answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput {
                    seed: 0,
                    hashcode: "".to_string()
                }),
                next_target: "".to_string()
            };
            send_message(&stream, result);
        }
        _ => {print!("Error")}
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