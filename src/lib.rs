pub mod gqlidl; // synthesized by LALRPOP
pub mod ast;

extern crate regex;

#[test]
fn scalar_no_description() {
    let def = gqlidl::parse_schema("scalar DateTime").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("scalar", def.typename.as_str());
    assert_eq!("DateTime", def.name);
}

#[test]
fn scalar_with_description() {
    let def = gqlidl::parse_schema("# An ISO-8601 encoded UTC date string.\nscalar DateTime").unwrap().pop().unwrap();

    assert_eq!("An ISO-8601 encoded UTC date string.", def.description);
    assert_eq!("scalar", def.typename.as_str());
    assert_eq!("DateTime", def.name);
}

#[test]
fn type_no_description() {
    let def = gqlidl::parse_schema("type CodeOfConduct {}").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CodeOfConduct", def.name);
}

#[test]
fn type_with_description() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct {}").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CodeOfConduct", def.name);
}

#[test]
fn type_with_field() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct { body: String }").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CodeOfConduct", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("", field.description.as_str());
    assert_eq!("String", field.typename.as_str());
    assert_eq!("body", field.name);
}


#[test]
fn type_with_field_and_description() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct { \n# The body of the CoC\n body: String }").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CodeOfConduct", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("The body of the CoC", field.description.as_str());
    assert_eq!("String", field.typename.as_str());
    assert_eq!("body", field.name);
}
