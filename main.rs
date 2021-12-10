mod libcob;
use libcob::{CobStr, cstr, cbuffer};
mod socket;
use socket::{connect};

extern "C" {
    fn entry(name: *const u8, size: *const u8, output: *mut u8);
}

fn main() {
    println!("BEGIN...");
    let msg = "What is this!!!!!";
    let output = cbuffer();
    unsafe {
        entry(cstr(msg), cstr(&msg.len().to_string()), output);
    }
    println!("\n\nBACK TO RUST");
    println!("Output: \"{}\"", CobStr::from_pointer(output).as_string());
    // println!("Output: {:?}", String::from_iter(slice::from_raw_parts(output, 100).iter().map(|x| *x as char)));

    // connect("irc.dot.org.es", 6667);
}
