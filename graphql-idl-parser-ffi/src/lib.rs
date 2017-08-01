#![crate_type = "dylib"]
extern crate libc;
extern crate graphql_idl_parser;

// use graphql_idl_parser::ast::{GraphQLScalar};

use libc::{c_char, size_t};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ffi::CStr;
use std::mem;

#[derive(Debug)]
#[repr(C)]
pub struct Scalar {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct Object {
    description: *const c_char,
    name: *const c_char,
    implements: Vec<*const c_char>,
}

pub enum GraphQLType {
    ScalarType(Scalar),
    // ObjectType(Object),
}

impl Scalar {
    pub fn new(typename: &str, description: Option<&str>, name: &str) -> Scalar {

      Scalar {
          typename: CString::new(typename).unwrap().into_raw(),
          description: match description {
              Some(d) => CString::new(d).unwrap().into_raw(),
              None => CString::new("").unwrap().into_raw()
          },
          name: CString::new(name).unwrap().into_raw(),
      }
    }
}

#[no_mangle]
#[allow(unused)]
pub extern fn gqlidl_parse_schema(schema: *const c_char, types: *mut *mut GraphQLType, types_len: *mut size_t) -> u8 {
    // Convert C string to Rust string
    let c_schema = unsafe {
        assert!(!schema.is_null());
        CStr::from_ptr(schema)
    };
    let r_schema = c_schema.to_str().unwrap();

    match graphql_idl_parser::gqlidl::parse_schema(r_schema) {
        Ok(vec) => {
            let mut tmp_vec: Vec<GraphQLType> = vec.into_iter().map(|mut v| {
                // v.shrink_to_fit();

                let s = match v.typename() {
                    "scalar" => {
                        return GraphQLType::ScalarType(Scalar::new(
                            v.typename(),
                            v.description(),
                            v.name())
                        );
                    },
                    _ => panic!("Unknown typename: {}", v.typename())
                };

                mem::forget(v);
                s
            }).collect();
            tmp_vec.shrink_to_fit();
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
