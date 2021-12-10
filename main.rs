#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod libcob;
use libcob::{cbuffer, cstr, CobStr};
mod ircclient;
use ircclient::{irc_connect, recv, send};

fn main() {
    match irc_connect("irc.dot.org.es", 6667, "rust_bot") {
        Some(mut client) => {
            // irc_send(&mut client, "JOIN #test");
            loop {
                match recv(&mut client) {
                    Ok(msg) => {
                        println!("{}", msg);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
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
        println!("BEGIN...");
        let msg = "Message from rust";
        let output = cbuffer();
        unsafe {
            hellocobol(cstr(msg), cstr(&msg.len().to_string()), output);
        }
        let msg = CobStr::from_pointer(output).as_string();
        println!("\n\nBACK TO RUST");
        println!("Output: \"{}\"", msg);
        assert_eq!(msg, "Hello from cobol");
    }
}
