extern crate graphql_idl_parser;

use graphql_idl_parser::type_definition::{GraphQLField, GraphQLValue};

use libc::{c_char, c_void, int32_t};
use std::ffi::{CString};
use std;

fn convert_optional_string_to_cstr(string: Option<&str>) -> *const c_char {
    match string {
        Some(string) => CString::new(string).unwrap().into_raw(),
        None => CString::new("").unwrap().into_raw(),
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Scalar {
    pub typename: *const c_char,
    pub name: *const c_char,
    pub description: *const c_char,
}

impl Clone for Scalar {
    fn clone(&self) -> Scalar {
        *self
    }
}

impl Scalar {
    pub fn new(typename: &str, name: &str, description: Option<&str>) -> Scalar {
        Scalar {
            typename: CString::new(typename).unwrap().into_raw(),
            name: CString::new(name).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Object {
    typename: *const c_char,
    name: *const c_char,
    description: *const c_char,
    implements: ArrayOfCStrings,
    fields: ArrayOfFields,
}

impl Clone for Object {
    fn clone(&self) -> Object {
        *self
    }
}

impl Object {
    pub fn new(
        typename: &str,
        name: &str,
        description: Option<&str>,
        implements: Option<Vec<String>>,
        fields: Option<Vec<graphql_idl_parser::type_definition::GraphQLField>>,
    ) -> Object {
        Object {
            typename: CString::new(typename).unwrap().into_raw(),
            name: CString::new(name).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            implements: match implements {
                None => ArrayOfCStrings::from_vec(vec![]),
                Some(implements) => ArrayOfCStrings::from_vec(implements),
            },
            fields: match fields {
                None => ArrayOfFields::from_vec(vec![]),
                Some(fields) => ArrayOfFields::from_vec(fields),
            },
        }
    }
}


#[derive(Copy)]
#[repr(C)]
pub struct Enum {
    typename: *const c_char,
    name: *const c_char,
    description: *const c_char,
    values: ArrayOfValues,
}

impl Clone for Enum {
    fn clone(&self) -> Enum {
        *self
    }
}

impl Enum {
    pub fn new(
        typename: &str,
        name: &str,
        description: Option<&str>,
        values: Option<Vec<graphql_idl_parser::type_definition::GraphQLValue>>,
    ) -> Enum {
        Enum {
            typename: CString::new(typename).unwrap().into_raw(),
            name: CString::new(name).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            values: match values {
                None => ArrayOfValues::from_vec(vec![]),
                Some(values) => ArrayOfValues::from_vec(values),
            },
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Interface {
    typename: *const c_char,
    name: *const c_char,
    description: *const c_char,
    fields: ArrayOfFields,
}

impl Clone for Interface {
    fn clone(&self) -> Interface {
        *self
    }
}

impl Interface {
    pub fn new(
        typename: &str,
        name: &str,
        description: Option<&str>,
        fields: Option<Vec<graphql_idl_parser::type_definition::GraphQLField>>,
    ) -> Interface {
        Interface {
            typename: CString::new(typename).unwrap().into_raw(),
            name: CString::new(name).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            fields: match fields {
                None => ArrayOfFields::from_vec(vec![]),
                Some(fields) => ArrayOfFields::from_vec(fields),
            },
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Union {
    typename: *const c_char,
    name: *const c_char,
    description: *const c_char,
    types: ArrayOfCStrings,
}

impl Clone for Union {
    fn clone(&self) -> Union {
        *self
    }
}

impl Union {
    pub fn new(
        typename: &str,
        name: &str,
        description: Option<&str>,
        types: Option<Vec<String>>,
    ) -> Union {
        Union {
            typename: CString::new(typename).unwrap().into_raw(),
            name: CString::new(name).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            types: match types {
                None => ArrayOfCStrings::from_vec(vec![]),
                Some(types) => ArrayOfCStrings::from_vec(types),
            },
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct InputObject {
    typename: *const c_char,
    name: *const c_char,
    description: *const c_char,
    fields: ArrayOfFields,
}

impl Clone for InputObject {
    fn clone(&self) -> InputObject {
        *self
    }
}

impl InputObject {
    pub fn new(
        typename: &str,
        name: &str,
        description: Option<&str>,
        fields: Option<Vec<graphql_idl_parser::type_definition::GraphQLField>>,
    ) -> InputObject {
        InputObject {
            typename: CString::new(typename).unwrap().into_raw(),
            name: CString::new(name).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            fields: match fields {
                None => ArrayOfFields::from_vec(vec![]),
                Some(fields) => ArrayOfFields::from_vec(fields),
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FieldType {
    name: *const c_char,
    type_info: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Argument {
    name: *const c_char,
    description: *const c_char,
    type_info: FieldType,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfArguments {
    length: int32_t,
    values: *const *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Field {
    name: *const c_char,
    description: *const c_char,
    type_info: FieldType,
    arguments: ArrayOfArguments,
    deprecated: bool,
    deprecation_reason: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfFields {
    length: int32_t,
    values: *const *const c_void,
}

impl ArrayOfFields {
    fn from_vec(vec: Vec<GraphQLField>) -> ArrayOfFields {
        let mut field_vec: Vec<Field> = vec![];
        let mut argument_vec: Vec<Argument> = vec![];

        for v in vec {
            match v.arguments() {
                None => {}
                Some(arguments) => for arg in arguments {
                    argument_vec.push(Argument {
                        name: CString::new(arg.name()).unwrap().into_raw(),
                        description: convert_optional_string_to_cstr(arg.description()),
                        type_info: FieldType {
                            name: CString::new(arg.typeinfo().name()).unwrap().into_raw(),
                            type_info: CString::new(arg.typeinfo().info()).unwrap().into_raw(),
                        },
                    });
                },
            }

            argument_vec.shrink_to_fit();

            let arguments = ArrayOfArguments {
                length: argument_vec.len() as int32_t,
                values: argument_vec.as_ptr() as *const *const c_void,
            };

            field_vec.push(Field {
                description: convert_optional_string_to_cstr(v.description()),
                name: CString::new(v.name()).unwrap().into_raw(),
                type_info: FieldType {
                    name: CString::new(v.typeinfo().name()).unwrap().into_raw(),
                    type_info: CString::new(v.typeinfo().info()).unwrap().into_raw(),
                },
                arguments: arguments,
                deprecated: v.deprecated(),
                deprecation_reason: convert_optional_string_to_cstr(v.deprecation_reason()),
            });
        }
        field_vec.shrink_to_fit();

        let array = ArrayOfFields {
            length: field_vec.len() as int32_t,
            values: field_vec.as_ptr() as *const *const c_void,
        };

        std::mem::forget(field_vec);
        std::mem::forget(argument_vec);
        array
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfCStrings {
    length: int32_t,
    values: *const *const c_char,
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
            values: cstr_vec.as_ptr() as *const *const c_char,
        };

        std::mem::forget(cstr_vec);
        array
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Value {
    name: *const c_char,
    description: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfValues {
    length: int32_t,
    values: *const *const c_void,
}


impl ArrayOfValues {
    fn from_vec(vec: Vec<GraphQLValue>) -> ArrayOfValues {
        let mut value_vec: Vec<Value> = vec![];

        for v in vec {
            value_vec.push(Value {
                name: CString::new(v.name()).unwrap().into_raw(),
                description: convert_optional_string_to_cstr(v.description()),
            });
        }
        value_vec.shrink_to_fit();

        let array = ArrayOfValues {
            length: value_vec.len() as int32_t,
            values: value_vec.as_ptr() as *const *const c_void,
        };

        std::mem::forget(value_vec);

        array
    }
}
