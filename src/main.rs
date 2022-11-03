use std::io;
use std::io::Write;
use std::net::TcpStream;
use byteorder::{ByteOrder, BigEndian};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected to the server!");

        let msg = "Hello".to_string();
        let size = msg.len();

        BigEndian::write_u32();

        stream.write(b"5 Hello")?;

        stream.flush()?;
        Ok(())
    } else {
        println!("Couldn't connect to server...");
        Ok(())
    }
}
