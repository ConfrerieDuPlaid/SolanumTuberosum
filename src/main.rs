use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected to the server!");

        let msg = "\"Hello\"".to_string();

        let bytes_msg = msg.as_bytes();
        let size = (bytes_msg.len() as u32).to_be_bytes();

        let result = stream.write(&[&size, bytes_msg].concat())?;
        println!("Write result : {:?}, message: {}", result, msg);

/*
        let mut buf_size = [0; 4];
        stream.try_clone()?.read(&mut buf_size).expect("Could not read stream message size");

        println!("{}", String::from_utf8_lossy(&buf_size));
*/
        stream.flush()?;
        Ok(())
    } else {
        println!("Couldn't connect to server...");
        Ok(())
    }
}
