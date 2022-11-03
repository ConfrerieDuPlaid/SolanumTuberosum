use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    if let Ok(stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
