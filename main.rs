#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod libcob;
use libcob::{cbuffer, cstr, cstr_fixed, CobStr};
mod ircclient;
use ircclient::{irc_connect, recv_loop, send};

pub fn nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

extern "C" {
    fn entry(name: *const u8, buffer: *const u8);
}

fn entry_h(msg: &str) -> String {
    let output = cbuffer();
    // TODO why is input screwing up the first bytes on cobol
    unsafe {
        entry(cstr_fixed(msg), output);
    }
    CobStr::from_pointer(output).as_string()
}

fn main() {
    // TODO read from arguments
    match irc_connect("irc.dot.org.es", 6667, "rust_bot") {
        Some(mut client) => {
            // irc_send(&mut client, "JOIN #test");
            recv_loop(&mut client, &entry_h);
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
