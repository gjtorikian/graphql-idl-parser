extern crate libc;
extern crate graphql_idl_parser;

use libc::{c_char, size_t};
use std::ffi::CString;
use std::ffi::CStr;
use std::mem;
use std::io::{self, Write};

#[derive(Copy, Debug)]
#[repr(C)]
pub struct Scalar {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
}

impl Clone for Scalar {
    fn clone(&self) -> Scalar { *self }
}

#[derive(Copy, Debug)]
#[repr(C)]
pub struct Object {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
    // implements: Vec<*const c_char>,
}

impl Clone for Object {
    fn clone(&self) -> Object { *self }
}

pub union GraphQLType {
    scalar: Scalar,
    object: Object,
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

impl Object {
    pub fn new(typename: &str, description: Option<&str>, name: &str) -> Object {
      Object {
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
                        let x = Scalar::new(
                            v.typename(),
                            v.description(),
                            v.name()
                        );
                        return GraphQLType { scalar: x };
                    },
                    "object" => {
                        let x = Object::new(
                            v.typename(),
                            v.description(),
                            v.name()
                        );
                        return GraphQLType { object: x };
                    },
                    _ => panic!("Unknown typename: {}", v.typename())
                };

                mem::forget(v);
                s
            }).collect();
            tmp_vec.shrink_to_fit();
            assert!(tmp_vec.len() == tmp_vec.capacity());

            // Return number of types
            unsafe { *types_len = tmp_vec.len() as size_t; }

            // Return pointer to data
            unsafe { *types = tmp_vec.as_mut_ptr(); }

            // Prevent memory from being deallocated
            mem::forget(tmp_vec);

            0
        },
        Err(err) => {
            writeln!(io::stderr(), "Catastrophic error: {:?}", err).unwrap();
            1
        }
    }
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }
