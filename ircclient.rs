#![allow(dead_code)]

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

const BUFF_SIZE : usize = 1024;

pub fn send(stream: &mut TcpStream, msg: &str) -> Result<(), ()> {
    let msg = format!("{}\r\n", msg);
    match stream.write(msg.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error sending message: {}", e);
            Err(())
        }
    }
}

pub fn recv(stream: &mut TcpStream) -> Result<String, ()> {
    let mut buff = [0; BUFF_SIZE];
    match stream.read(&mut buff) {
        Ok(msg) => {
            let msg = from_utf8(&buff).unwrap();
            let msg = msg.trim();
            Ok(msg.to_string())
        },
        Err(_) => {
            println!("Error receiving message");
            Err(())
        }
    }
}

pub fn irc_connect(address: &str, port: u16, nick: &str) -> Option<TcpStream> {
    println!("Connecting to {} on port {}", address, port);
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to {}:{}", address, port);
            println!("Authenticating...");
            send(&mut stream, format!("NICK {}", nick).as_str()).unwrap();
            send(&mut stream, format!("USER {} {} {} :{}", nick, nick, nick, nick).as_str()).unwrap();
            // TODO: ping cookie
            Some(stream)
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            None
        }
    }
}
