pub mod gqlidl; // synthesized by LALRPOP
pub mod ast;

#[cfg(test)]
mod tests;

extern crate regex;
extern crate libc;

use libc::{c_char};
use std::fs::File;
use std::io::Read;
use std::ffi::CStr;
use ast::GraphQLType;

#[no_mangle]
#[allow(unused)]
pub extern fn parse(s: *const c_char) -> Vec<GraphQLType> {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let filename = c_str.to_str().unwrap();

    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    gqlidl::parse_schema(contents.as_str()).unwrap()
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }
