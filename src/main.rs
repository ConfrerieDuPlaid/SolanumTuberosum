
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Hello,
    Welcome {
        version: u8,
    },
    Subscribe {
        name: String,
    },

}

fn main() {
    println!("Hello, world!");


    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            stream.set_nonblocking(true);
            let message = Message::Hello;
            send_message(&stream, message);
            receive_messages(&stream);
        },
        Err(_) => panic!("Could not connect to server "),
    }

}

fn receive_messages(mut stream: &TcpStream){
    loop {
        let mut v = Vec::<u8>::new();
        stream.read_to_end(&mut v);
        let str = String::from_utf8_lossy(&v);
        if str != "" {
            println!("{str:?}");
        }
    }
}


fn send_message(mut stream: &TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let bytes_message = message.as_bytes();
        let message_size = bytes_message.len() as u32;
        let message_length_as_bytes = message_size.to_be_bytes();
        stream.write(&message_length_as_bytes).unwrap();
        let result = stream.write(bytes_message).unwrap();
        println!("result : {}, message: {}", result, message);
    }
}