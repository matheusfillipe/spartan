use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

pub fn connect(address: &str, port: u16) {
    println!("Connecting to {} on port {}", address, port);
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to {}:{}", address, port);

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
