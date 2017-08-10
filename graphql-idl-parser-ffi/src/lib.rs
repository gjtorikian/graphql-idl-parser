extern crate libc;
extern crate graphql_idl_parser;

mod types;

use types::*;

use libc::{c_char, size_t};
use std::ffi::CStr;
use std::mem;
use std::io::{self, Write};

#[repr(C)]
pub union GraphQLType {
    scalar: Scalar,
    object: Object,
    enum_type: Enum,
    interface: Interface,
    union: Union,
    input_object: InputObject,
}

#[no_mangle]
#[allow(unused)]
pub extern "C" fn gqlidl_parse_schema(
    schema: *const c_char,
    types: *mut *mut GraphQLType,
    types_len: *mut size_t,
) -> u8 {
    // Convert C string to Rust string
    let c_schema = unsafe {
        assert!(!schema.is_null());
        CStr::from_ptr(schema)
    };
    let r_schema = c_schema.to_str().unwrap();

    match graphql_idl_parser::gqlidl::parse_schema(r_schema) {
        Ok(vec) => {
            let mut tmp_vec: Vec<GraphQLType> = vec.into_iter()
                .map(|mut v| {
                    let s = match v.typename() {
                        "scalar" => {
                            let d = Scalar::new(v.typename(), v.name(), v.description());
                            return GraphQLType { scalar: d };
                        }
                        "object" => {
                            let d = Object::new(
                                v.typename(),
                                v.name(),
                                v.description(),
                                v.implements(),
                                v.fields(),
                            );
                            return GraphQLType { object: d };
                        }
                        "enum" => {
                            let d = Enum::new(v.typename(), v.name(), v.description(), v.values());
                            return GraphQLType { enum_type: d };
                        }
                        "interface" => {
                            let d =
                                Interface::new(v.typename(), v.name(), v.description(), v.fields());
                            return GraphQLType { interface: d };
                        }
                        "union" => {
                            let d = Union::new(v.typename(), v.name(), v.description(), v.types());
                            return GraphQLType { union: d };
                        }
                        "input_object" => {
                            let d = InputObject::new(
                                v.typename(),
                                v.name(),
                                v.description(),
                                v.fields(),
                            );
                            return GraphQLType { input_object: d };
                        }
                        _ => panic!("Unknown typename: {}", v.typename()),
                    };

                    mem::forget(v);
                    s
                })
                .collect();

            tmp_vec.shrink_to_fit();
            assert!(tmp_vec.len() == tmp_vec.capacity());

            // Return number of types
            unsafe {
                *types_len = tmp_vec.len() as size_t;
            }

            // Return pointer to data
            unsafe {
                *types = tmp_vec.as_mut_ptr();
            }

            // Prevent memory from being deallocated
            mem::forget(tmp_vec);

            0
        }
        Err(err) => {
            writeln!(io::stderr(), "Catastrophic error: {:?}", err).unwrap();
            1
        }
    }
}
