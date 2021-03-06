extern crate graphql_idl_parser;

use graphql_idl_parser::type_definition::{GraphQLDirective, GraphQLField, GraphQLValue};

use libc::{c_char, c_void, size_t};
use std::ffi::CString;
use std::{mem, ptr};

fn convert_optional_string_to_cstr(string: Option<&str>) -> *const c_char {
    match string {
        Some(string) => CString::new(string).unwrap().into_raw(),
        None => ptr::null(),
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Scalar {
    pub typename: *const c_char,
    pub description: *const c_char,
    pub name: *const c_char,
}

impl Clone for Scalar {
    fn clone(&self) -> Scalar {
        *self
    }
}

impl Scalar {
    pub fn new(typename: &str, description: Option<&str>, name: &str) -> Scalar {
        Scalar {
            typename: CString::new(typename).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            name: CString::new(name).unwrap().into_raw(),
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Object {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
    implements: ArrayOfCStrings,
    directives: ArrayOfDirectives,
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
        description: Option<&str>,
        name: &str,
        implements: Option<Vec<String>>,
        directives: Option<Vec<GraphQLDirective>>,
        fields: Option<Vec<GraphQLField>>,
    ) -> Object {
        Object {
            typename: CString::new(typename).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            name: CString::new(name).unwrap().into_raw(),
            implements: match implements {
                None => ArrayOfCStrings::from_vec(vec![]),
                Some(implements) => ArrayOfCStrings::from_vec(implements),
            },
            directives: match directives {
                None => ArrayOfDirectives::from_vec(vec![]),
                Some(directives) => ArrayOfDirectives::from_vec(directives),
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
    description: *const c_char,
    name: *const c_char,
    directives: ArrayOfDirectives,
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
        description: Option<&str>,
        name: &str,
        directives: Option<Vec<GraphQLDirective>>,
        values: Option<Vec<GraphQLValue>>,
    ) -> Enum {
        Enum {
            typename: CString::new(typename).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            name: CString::new(name).unwrap().into_raw(),
            directives: match directives {
                None => ArrayOfDirectives::from_vec(vec![]),
                Some(directives) => ArrayOfDirectives::from_vec(directives),
            },
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
    description: *const c_char,
    name: *const c_char,
    directives: ArrayOfDirectives,
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
        description: Option<&str>,
        name: &str,
        directives: Option<Vec<GraphQLDirective>>,
        fields: Option<Vec<GraphQLField>>,
    ) -> Interface {
        Interface {
            typename: CString::new(typename).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            name: CString::new(name).unwrap().into_raw(),
            directives: match directives {
                None => ArrayOfDirectives::from_vec(vec![]),
                Some(directives) => ArrayOfDirectives::from_vec(directives),
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
pub struct Union {
    typename: *const c_char,
    description: *const c_char,
    name: *const c_char,
    directives: ArrayOfDirectives,
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
        description: Option<&str>,
        name: &str,
        directives: Option<Vec<GraphQLDirective>>,
        types: Option<Vec<String>>,
    ) -> Union {
        Union {
            typename: CString::new(typename).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            name: CString::new(name).unwrap().into_raw(),
            directives: match directives {
                None => ArrayOfDirectives::from_vec(vec![]),
                Some(directives) => ArrayOfDirectives::from_vec(directives),
            },
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
    description: *const c_char,
    name: *const c_char,
    directives: ArrayOfDirectives,
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
        description: Option<&str>,
        name: &str,
        directives: Option<Vec<GraphQLDirective>>,
        fields: Option<Vec<GraphQLField>>,
    ) -> InputObject {
        InputObject {
            typename: CString::new(typename).unwrap().into_raw(),
            description: convert_optional_string_to_cstr(description),
            name: CString::new(name).unwrap().into_raw(),
            directives: match directives {
                None => ArrayOfDirectives::from_vec(vec![]),
                Some(directives) => ArrayOfDirectives::from_vec(directives),
            },
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
    description: *const c_char,
    name: *const c_char,
    type_info: FieldType,
    default_value: *const c_char,
    directives: ArrayOfDirectives,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfArguments {
    length: size_t,
    values: *const *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Field {
    description: *const c_char,
    name: *const c_char,
    type_info: FieldType,
    arguments: ArrayOfArguments,
    directives: ArrayOfDirectives,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfFields {
    length: size_t,
    values: *const *const c_void,
}

impl ArrayOfFields {
    fn from_vec(vec: Vec<GraphQLField>) -> ArrayOfFields {
        let mut field_vec: Vec<Field> = vec![];

        for v in vec {
            let mut argument_vec: Vec<Argument> = vec![];
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
                        default_value: convert_optional_string_to_cstr(arg.default()),
                        directives: match v.directives() {
                            None => ArrayOfDirectives::from_vec(vec![]),
                            Some(directives) => ArrayOfDirectives::from_vec(directives),
                        },
                    });
                },
            }

            argument_vec.shrink_to_fit();

            let arguments = ArrayOfArguments {
                length: argument_vec.len() as size_t,
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
                directives: match v.directives() {
                    None => ArrayOfDirectives::from_vec(vec![]),
                    Some(directives) => ArrayOfDirectives::from_vec(directives),
                },
            });

            mem::forget(argument_vec);
        }
        field_vec.shrink_to_fit();

        let array = ArrayOfFields {
            length: field_vec.len() as size_t,
            values: field_vec.as_ptr() as *const *const c_void,
        };

        mem::forget(field_vec);
        array
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfCStrings {
    length: size_t,
    values: *const *const c_char,
}

impl ArrayOfCStrings {
    fn from_vec(vec: Vec<String>) -> ArrayOfCStrings {
        let mut cstr_vec: Vec<*const c_char> = vec![];
        for s in vec {
            let cstr = CString::new(s).unwrap();
            cstr_vec.push(cstr.into_raw() as *const c_char);
        }
        cstr_vec.shrink_to_fit();

        let array = ArrayOfCStrings {
            length: cstr_vec.len() as size_t,
            values: cstr_vec.as_ptr() as *const *const c_char,
        };

        mem::forget(cstr_vec);
        array
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Value {
    description: *const c_char,
    name: *const c_char,
    directives: ArrayOfDirectives,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfValues {
    length: size_t,
    values: *const *const c_void,
}

impl ArrayOfValues {
    fn from_vec(vec: Vec<GraphQLValue>) -> ArrayOfValues {
        let mut value_vec: Vec<Value> = vec![];

        for v in vec {
            value_vec.push(Value {
                name: CString::new(v.name()).unwrap().into_raw(),
                description: convert_optional_string_to_cstr(v.description()),
                directives: match v.directives() {
                    None => ArrayOfDirectives::from_vec(vec![]),
                    Some(directives) => ArrayOfDirectives::from_vec(directives),
                },
            });
        }
        value_vec.shrink_to_fit();

        let array = ArrayOfValues {
            length: value_vec.len() as size_t,
            values: value_vec.as_ptr() as *const *const c_void,
        };

        mem::forget(value_vec);

        array
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Directive {
    name: *const c_char,
    arguments: ArrayOfDirectiveArguments,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfDirectives {
    length: size_t,
    values: *const *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DirectiveArgument {
    name: *const c_char,
    value: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ArrayOfDirectiveArguments {
    length: size_t,
    values: *const *const c_void,
}

impl ArrayOfDirectives {
    fn from_vec(vec: Vec<GraphQLDirective>) -> ArrayOfDirectives {
        let mut value_vec: Vec<Directive> = vec![];

        for v in vec {
            let mut argument_vec: Vec<DirectiveArgument> = vec![];
            match v.arguments() {
                None => {}
                Some(arguments) => for arg in arguments {
                    argument_vec.push(DirectiveArgument {
                        name: CString::new(arg.name()).unwrap().into_raw(),
                        value: convert_optional_string_to_cstr(arg.value()),
                    });
                },
            }

            argument_vec.shrink_to_fit();

            let arguments = ArrayOfDirectiveArguments {
                length: argument_vec.len() as size_t,
                values: argument_vec.as_ptr() as *const *const c_void,
            };

            value_vec.push(Directive {
                name: CString::new(v.name()).unwrap().into_raw(),
                arguments: arguments,
            });

            mem::forget(argument_vec);
        }
        value_vec.shrink_to_fit();

        let array = ArrayOfDirectives {
            length: value_vec.len() as size_t,
            values: value_vec.as_ptr() as *const *const c_void,
        };

        mem::forget(value_vec);

        array
    }
}
