extern crate libc;
extern crate graphql_idl_parser;

use libc::{c_char, size_t, int32_t, int64_t, c_void};
use std::ffi::CString;
use std::ffi::CStr;
use std::mem;
use std::io::{self, Write};

#[derive(Copy)]
#[repr(C)]
pub struct Scalar {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
}

impl Clone for Scalar {
    fn clone(&self) -> Scalar { *self }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfCStrings {
    length: int32_t,
    values: *const *const c_char
}

impl ArrayOfCStrings {
    fn from_vec(vec: Vec<std::string::String>) -> ArrayOfCStrings {
        let mut cstr_vec: Vec<*const c_char> = vec![];
        for s in vec {
            let cstr = CString::new(s).unwrap();
            cstr_vec.push(cstr.into_raw() as *const c_char);
        }
        cstr_vec.shrink_to_fit();

        let array = ArrayOfCStrings {
            length: cstr_vec.len() as int32_t,
            values: cstr_vec.as_ptr() as *const *const c_char
        };

        std::mem::forget(cstr_vec);
        array
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Object {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
    implements: ArrayOfCStrings,
}

impl Clone for Object {
    fn clone(&self) -> Object { *self }
}

#[repr(C)]
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
    pub fn new(typename: &str, description: Option<&str>, name: &str, implements: Option<Vec<String>>) -> Object {
      Object {
          typename: CString::new(typename).unwrap().into_raw(),
          description: match description {
              Some(d) => CString::new(d).unwrap().into_raw(),
              None => CString::new("").unwrap().into_raw()
          },
          name: CString::new(name).unwrap().into_raw(),
          implements: match implements {
                      None => {
                          ArrayOfCStrings::from_vec(vec![])
                      }
                      Some(vec) => {
                          ArrayOfCStrings::from_vec(vec)
                      }
                  }
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
                            v.name(),
                            v.implements()
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
