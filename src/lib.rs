extern crate regex;
extern crate libc;

#[macro_use]
mod macros;
pub mod gqlidl; // synthesized by LALRPOP
pub mod type_definition;


#[cfg(test)]
mod tests;

use libc::c_char;
use std::fs::File;
use std::io::Read;
use std::ffi::CStr;
use type_definition::TypeDefinition;

#[no_mangle]
#[allow(unused)]
pub extern "C" fn parse(s: *const c_char) -> Vec<TypeDefinition> {
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
pub extern "C" fn fix_linking_when_not_using_stdlib() {
    panic!()
}
