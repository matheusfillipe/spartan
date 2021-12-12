#![allow(dead_code)]

use std::iter::FromIterator;
use std::slice;

pub const BUFF_SIZE: usize = 1024;
pub const STR_SIZE: usize = 512;

pub enum CobDataType<'a> {
    Value(&'a str),
    Buffer(*mut u8),
}

pub struct CobStr<'a> {
    pub value: CobDataType<'a>,
    size: usize,
}

impl<'a> CobStr<'a> {
    pub fn new(s: &'a str) -> Self {
        let size = String::from(s).len();
        let value = CobDataType::Value(s);
        CobStr { value, size }
    }
    pub fn from_pointer(buffer: *mut u8) -> Self {
        let value = CobDataType::Buffer(buffer);
        CobStr { value, size: 0 }
    }
    pub fn as_string(&self) -> String {
        match self.value {
            CobDataType::Value(value) => String::from(value),
            CobDataType::Buffer(ptr) => {
                let raw_slice = unsafe { slice::from_raw_parts(ptr, STR_SIZE) };
                let output = String::from_iter(raw_slice.iter().map(|x| *x as char));
                String::from(output.trim())
            }
        }
    }
    pub fn as_array(&self) -> *const u8 {
        match self.value {
            CobDataType::Value(ptr) => ptr.as_ptr(),
            CobDataType::Buffer(_) => {
                panic!("Tried to read a value from a pointer. Use as_buffer instead.");
            }
        }
    }
    pub fn as_buffer(&self) -> *mut u8 {
        match self.value {
            CobDataType::Buffer(ptr) => ptr,
            CobDataType::Value(value) => {
                println!("value error: {}", value);
                panic!("Tried to use a value as a pointer! Use as_array instead.");
            }
        }
    }
}

pub fn cstr(value: &str) -> *const u8 {
    CobStr::new(value).as_array()
}

pub fn cstr_fixed(value: &str) -> *const u8 {
    let formatted: String;
    if value.len() > STR_SIZE {
        formatted = value[0..STR_SIZE].to_string();
    } else {
        formatted = format!("{: <1$}", value, STR_SIZE - 1);
    };

    print!(
        "------------------------------------------------------------------------------------\n"
    );
    println!("Sending to cobol: {:?}", formatted);
    CobStr::new(&formatted).as_array()
}

pub fn cbuffer() -> *mut u8 {
    let mut buffer = [0; BUFF_SIZE];
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}
