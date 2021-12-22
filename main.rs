#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod libcob;
use libcob::{cbuffer, cstr, cstr_fixed, CobStr};
mod ircclient;
use ircclient::{irc_connect, recv_loop, send};

extern "C" {
    fn entry(name: *const u8, buffer: *const u8);
    fn echo(name: *const u8, buffer: *const u8);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--echo".to_string()) {
        let mut input = String::new();
        loop {
            std::io::stdin().read_line(&mut input).unwrap();
            println!("{}", cobcall!(&echo, &input));
        }
    }
    if args.contains(&"--help".to_string()) {
        //|| args.len() < 2 {
        println!("Usage: {} <server> <port>", args[0]);
        return;
    }
    match irc_connect("irc.dot.org.es", 6667, "rust_bot") {
        Some(mut client) => {
            // irc_send(&mut client, "JOIN #test");
            recv_loop(&mut client, cobcallable!(&entry));
        }
        None => {
            println!("Error connecting to IRC server");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" {
        fn hellocobol(name: *const u8, size: *const u8, output: *mut u8);
    }

    #[test]
    fn cobol_hello_world() {
        let msg = "Message from rust";
        let output = cbuffer();
        unsafe {
            hellocobol(cstr(msg), cstr(&msg.len().to_string()), output);
        }
        let msg = CobStr::from_pointer(output).as_string();
        println!("Output: \"{}\"", msg);
        assert_eq!(msg, "Hello from cobol");
    }
}
