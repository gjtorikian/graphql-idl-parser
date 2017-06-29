pub mod gqlidl; // synthesized by LALRPOP
pub mod ast;

extern crate regex;

#[test]
fn scalar_no_description() {
    let defs = gqlidl::parse_schema("scalar DateTime").unwrap().pop().unwrap();

    assert_eq!("", defs.description.as_str());
    assert_eq!("scalar", defs.gql_type.as_str());
    assert_eq!("DateTime", defs.name);
}

#[test]
fn scalar_with_description() {
    let defs = gqlidl::parse_schema("# An ISO-8601 encoded UTC date string.\nscalar DateTime").unwrap().pop().unwrap();

    assert_eq!("An ISO-8601 encoded UTC date string.", defs.description);
    assert_eq!("scalar", defs.gql_type.as_str());
    assert_eq!("DateTime", defs.name);
}

#[test]
fn type_no_description() {
    let defs = gqlidl::parse_schema("type CodeOfConduct").unwrap().pop().unwrap();

    assert_eq!("", defs.description.as_str());
    assert_eq!("object", defs.gql_type.as_str());
    assert_eq!("CodeOfConduct", defs.name);
}
