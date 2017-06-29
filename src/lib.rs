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
fn type_with_one_implements() {
    let def = gqlidl::parse_schema("type PushAllowance implements Node {}").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("PushAllowance", def.name);

    let mut d = def;
    let implement = d.implements.pop().unwrap();

    assert_eq!("Node", implement);
}

#[test]
fn type_with_multiple_implements() {
    let def = gqlidl::parse_schema("type Release implements Node, UniformResourceLocatable {}").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("Release", def.name);

    let mut d = def;
    let mut implement = d.implements.remove(0);

    assert_eq!("Node", implement);
    implement = d.implements.remove(0);
    assert_eq!("UniformResourceLocatable", implement);
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
    assert_eq!("body", field.name);
    assert_eq!("String", field.fieldtype.name.as_str());
    assert_eq!("", field.fieldtype.info.as_str());
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
    assert_eq!("body", field.name);
    assert_eq!("String", field.fieldtype.name.as_str());
    assert_eq!("", field.fieldtype.info.as_str());
}

#[test]
fn type_with_required_field() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct { key: String! }").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CodeOfConduct", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("", field.description.as_str());
    assert_eq!("key", field.name);
    assert_eq!("String", field.fieldtype.name.as_str());
    assert_eq!("!", field.fieldtype.info.as_str());
}

#[test]
fn type_with_nullable_field_list() {
    let def = gqlidl::parse_schema("type CommitCommentConnection { edges: [CommitCommentEdge] }").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CommitCommentConnection", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("", field.description.as_str());
    assert_eq!("edges", field.name);
    assert_eq!("CommitCommentEdge", field.fieldtype.name.as_str());
    assert_eq!("[]", field.fieldtype.info.as_str());
}

#[test]
fn type_with_non_nullable_field_non_nullable_list() {
    let def = gqlidl::parse_schema("type CommitComment { viewerCannotUpdateReasons: [CommentCannotUpdateReason!]! }").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("CommitComment", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("", field.description.as_str());
    assert_eq!("viewerCannotUpdateReasons", field.name);
    assert_eq!("CommentCannotUpdateReason", field.fieldtype.name.as_str());
    assert_eq!("[!]!", field.fieldtype.info.as_str());
}

#[test]
fn type_with_nullable_field_non_nullable_list() {
    let def = gqlidl::parse_schema("type PullRequest { suggestedReviewers: [SuggestedReviewer]! }").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("PullRequest", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("", field.description.as_str());
    assert_eq!("suggestedReviewers", field.name);
    assert_eq!("SuggestedReviewer", field.fieldtype.name.as_str());
    assert_eq!("[]!", field.fieldtype.info.as_str());
}

#[test]
fn type_with_nullable_connection() {
    let def = gqlidl::parse_schema("type User {
        # A list of users the given user is followed by.
        followers(
          # Returns the elements in the list that come after the specified global ID.
          after: String

          # Returns the first _n_ elements from the list.
          first: Int!
        ): FollowerConnection!
    }
    ").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("User", def.name);

    let mut d = def;
    let connection = d.connections.pop().unwrap();

    assert_eq!("A list of users the given user is followed by.", connection.description.as_str());
    assert_eq!("followers", connection.name);
    assert_eq!("FollowerConnection", connection.fieldtype.name.as_str());
    assert_eq!("!", connection.fieldtype.info.as_str());

    let mut c = connection;
    let mut argument = c.arguments.remove(0);

    assert_eq!("Returns the elements in the list that come after the specified global ID.", argument.description);
    assert_eq!("String", argument.fieldtype.name.as_str());
    assert_eq!("", argument.fieldtype.info.as_str());

    argument = c.arguments.remove(0);

    assert_eq!("Returns the first _n_ elements from the list.", argument.description);
    assert_eq!("Int", argument.fieldtype.name.as_str());
    assert_eq!("!", argument.fieldtype.info.as_str());
}

#[test]
fn type_with_deprecated_field() {
    let def = gqlidl::parse_schema("type User {
        databaseId: Int @deprecated
    }
    ").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("User", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("databaseId", field.name);
    assert_eq!("Int", field.fieldtype.name.as_str());
    assert_eq!("", field.fieldtype.info.as_str());
    assert_eq!(true, field.deprecated);
    assert_eq!("", field.deprecated_reason.as_str());
}

#[test]
fn type_with_deprecated_field_and_reason() {
    let def = gqlidl::parse_schema("type User {
        databaseId: Int @deprecated(reason: \"Exposed database IDs will eventually be removed in favor of global Relay IDs.\")
    }
    ").unwrap().pop().unwrap();

    assert_eq!("", def.description.as_str());
    assert_eq!("object", def.typename.as_str());
    assert_eq!("User", def.name);

    let mut d = def;
    let field = d.fields.pop().unwrap();

    assert_eq!("databaseId", field.name);
    assert_eq!("Int", field.fieldtype.name.as_str());
    assert_eq!("", field.fieldtype.info.as_str());
    assert_eq!(true, field.deprecated);
    assert_eq!("Exposed database IDs will eventually be removed in favor of global Relay IDs.", field.deprecated_reason.as_str());
}
