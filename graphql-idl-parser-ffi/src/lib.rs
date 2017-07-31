#![crate_type = "dylib"]
extern crate libc;
extern crate graphql_idl_parser;

use graphql_idl_parser::ast::{GraphQLScalar, GraphQLType};

use libc::{c_char, size_t};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ffi::CStr;
use std::mem;

#[derive(Debug)]
#[repr(C)]
pub struct Scalar {
    description: *mut c_char,
    name: *mut c_char,
}

#[no_mangle]
#[allow(unused)]
pub extern fn gqlidl_parse_schema(s: *const c_char, types: *mut *mut Scalar, types_len: *mut size_t) -> u8 {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let filename = c_str.to_str().unwrap();

    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    match graphql_idl_parser::gqlidl::parse_schema(contents.as_str()) {
        Ok(vec) => {
            let mut tmp_vec: Vec<Scalar> = vec.into_iter().map(|mut v| {
                // v.shrink_to_fit();
                let d = match v.description() {
                    Some(desc) => vec![desc],
                    None       => Vec::new()
                };

                let s = Scalar {
                    description: CString::new(d.join(" ")).unwrap().into_raw(),
                    name: CString::new(v.name()).unwrap().into_raw()
                };
                mem::forget(v);
                s
            }).collect();
            // tmp_vec.shrink_to_fit();
            assert!(tmp_vec.len() == tmp_vec.capacity());

            // println!("WOW: {}", tmp_vec[0].name.into_string.unwrap());
            // Return number of types
            unsafe { *types_len = tmp_vec.len() as size_t; }

            // Return pointer to data
            unsafe { *types = tmp_vec.as_mut_ptr(); }

            // Prevent memory from being deallocated
            mem::forget(tmp_vec);

            0
        },
        Err(_) => 1
    }
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }
