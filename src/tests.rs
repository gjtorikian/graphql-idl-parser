use gqlidl;

#[test]
fn scalar_no_description() {
    let def = gqlidl::parse_schema("scalar DateTime").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("scalar", def.typename());
    assert_eq!("DateTime", def.name());
}

#[test]
fn scalar_with_description() {
    let def = gqlidl::parse_schema("# An ISO-8601 encoded UTC date string.\nscalar DateTime").unwrap().pop().unwrap();

    assert_eq!("An ISO-8601 encoded UTC date string.", def.description().unwrap());
    assert_eq!("scalar", def.typename());
    assert_eq!("DateTime", def.name());
}

#[test]
fn type_no_description() {
    let def = gqlidl::parse_schema("type CodeOfConduct {}").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("CodeOfConduct", def.name());
}

#[test]
fn type_with_description() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct {}").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description().unwrap());
    assert_eq!("object", def.typename());
    assert_eq!("CodeOfConduct", def.name());
    assert_eq!(None, def.implements());
}

#[test]
fn type_with_one_implements() {
    let def = gqlidl::parse_schema("type PushAllowance implements Node {}").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("PushAllowance", def.name());

    let implement = def.implements().unwrap().pop().unwrap();

    assert_eq!("Node", implement);
}

#[test]
fn type_with_multiple_implements() {
    let def = gqlidl::parse_schema("type Release implements Node, UniformResourceLocatable {}").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("Release", def.name());

    let mut implement = def.implements().unwrap().remove(0);

    assert_eq!("Node", implement);
    implement = def.implements().unwrap().remove(1);
    assert_eq!("UniformResourceLocatable", implement);
}


#[test]
fn type_with_field() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct { body: String }").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description().unwrap());
    assert_eq!("object", def.typename());
    assert_eq!("CodeOfConduct", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!(None, field.description());
    assert_eq!("body", field.name());
    assert_eq!("String", field.typeinfo().name());
    assert_eq!("", field.typeinfo().info());
}

#[test]
fn type_with_field_and_description() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct { \n# The body of the CoC\n body: String }").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description().unwrap());
    assert_eq!("object", def.typename());
    assert_eq!("CodeOfConduct", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!("The body of the CoC", field.description().unwrap());
    assert_eq!("body", field.name());
    assert_eq!("String", field.typeinfo().name());
    assert_eq!("", field.typeinfo().info());
}

#[test]
fn type_with_required_field() {
    let def = gqlidl::parse_schema("# The Code of Conduct for a repository\ntype CodeOfConduct { key: String! }").unwrap().pop().unwrap();

    assert_eq!("The Code of Conduct for a repository", def.description().unwrap());
    assert_eq!("object", def.typename());
    assert_eq!("CodeOfConduct", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!(None, field.description());
    assert_eq!("key", field.name());
    assert_eq!("String", field.typeinfo().name());
    assert_eq!("!", field.typeinfo().info());
}

#[test]
fn type_with_nullable_field_list() {
    let def = gqlidl::parse_schema("type CommitCommentConnection { edges: [CommitCommentEdge] }").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("CommitCommentConnection", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!(None, field.description());
    assert_eq!("edges", field.name());
    assert_eq!("CommitCommentEdge", field.typeinfo().name());
    assert_eq!("[]", field.typeinfo().info());
}

#[test]
fn type_with_non_nullable_field_non_nullable_list() {
    let def = gqlidl::parse_schema("type CommitComment { viewerCannotUpdateReasons: [CommentCannotUpdateReason!]! }").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("CommitComment", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!(None, field.description());
    assert_eq!("viewerCannotUpdateReasons", field.name());
    assert_eq!("CommentCannotUpdateReason", field.typeinfo().name());
    assert_eq!("[!]!", field.typeinfo().info());
}

#[test]
fn type_with_nullable_field_non_nullable_list() {
    let def = gqlidl::parse_schema("type PullRequest { suggestedReviewers: [SuggestedReviewer]! }").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("PullRequest", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!(None, field.description());
    assert_eq!("suggestedReviewers", field.name());
    assert_eq!("SuggestedReviewer", field.typeinfo().name());
    assert_eq!("[]!", field.typeinfo().info());
}

#[test]
fn type_with_non_nullable_connection() {
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

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("User", def.name());

    let connection = def.fields().unwrap().pop().unwrap();

    assert_eq!("A list of users the given user is followed by.", connection.description().unwrap());
    assert_eq!("followers", connection.name());
    assert_eq!("FollowerConnection", connection.typeinfo().name());
    assert_eq!("!", connection.typeinfo().info());

    let mut argument = connection.arguments().unwrap().remove(0);

    assert_eq!("Returns the elements in the list that come after the specified global ID.", argument.description().unwrap());
    assert_eq!("String", argument.typeinfo().name());
    assert_eq!("", argument.typeinfo().info());

    argument = connection.arguments().unwrap().remove(1);

    assert_eq!("Returns the first _n_ elements from the list.", argument.description().unwrap());
    assert_eq!("Int", argument.typeinfo().name());
    assert_eq!("!", argument.typeinfo().info());
}

#[test]
fn type_with_deprecated_field() {
    let def = gqlidl::parse_schema("type User {
        databaseId: Int @deprecated
    }
    ").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("User", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!("databaseId", field.name());
    assert_eq!("Int", field.typeinfo().name());
    assert_eq!("", field.typeinfo().info());
    assert_eq!(true, field.deprecated());
    assert_eq!(None, field.deprecation_reason());
}

#[test]
fn type_with_deprecated_field_and_reason() {
    let def = gqlidl::parse_schema("type User {
        databaseId: Int @deprecated(reason: \"Exposed database IDs will eventually be removed in favor of global Relay IDs.\")
    }
    ").unwrap().pop().unwrap();

    assert_eq!(None, def.description());
    assert_eq!("object", def.typename());
    assert_eq!("User", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!("databaseId", field.name());
    assert_eq!("Int", field.typeinfo().name());
    assert_eq!("", field.typeinfo().info());
    assert_eq!(true, field.deprecated());
    assert_eq!("Exposed database IDs will eventually be removed in favor of global Relay IDs.", field.deprecation_reason().unwrap());
}

#[test]
fn enum_with_fields() {
    let def = gqlidl::parse_schema("
    # State of the project; either 'open' or 'closed'
    enum ProjectState {
      # The project is closed.
      CLOSED

      # The project is open.
      OPEN
    }
    ").unwrap().pop().unwrap();

    assert_eq!("State of the project; either 'open' or 'closed'", def.description().unwrap());
    assert_eq!("enum", def.typename());
    assert_eq!("ProjectState", def.name());

    let mut value = def.values().unwrap().remove(0);

    assert_eq!("The project is closed.", value.description().unwrap());
    assert_eq!("CLOSED", value.name());

    value = def.values().unwrap().remove(1);

    assert_eq!("The project is open.", value.description().unwrap());
    assert_eq!("OPEN", value.name());
}

#[test]
fn interface_with_field() {
    let def = gqlidl::parse_schema("
    # An object that can be closed
    interface Closable {
      # `true` if the object is closed (definition of closed may depend on type)
      closed: Boolean!
    }
    ").unwrap().pop().unwrap();

    assert_eq!("An object that can be closed", def.description().unwrap());
    assert_eq!("interface", def.typename());
    assert_eq!("Closable", def.name());

    let field = def.fields().unwrap().pop().unwrap();

    assert_eq!("`true` if the object is closed (definition of closed may depend on type)", field.description().unwrap());
    assert_eq!("closed", field.name());
    assert_eq!("Boolean", field.typeinfo().name());
    assert_eq!("!", field.typeinfo().info());
}

#[test]
fn union_with_descriptions() {
    let def = gqlidl::parse_schema("
    # Any referencable object
    union ReferencedSubject = Issue | PullRequest
    ").unwrap().pop().unwrap();

    assert_eq!("Any referencable object", def.description().unwrap());
    assert_eq!("union", def.typename());
    assert_eq!("ReferencedSubject", def.name());

    let mut _type = def.types().unwrap().remove(0);

    assert_eq!("Issue", _type);

    _type = def.types().unwrap().remove(1);

    assert_eq!("PullRequest", _type);
}

#[test]
fn input_object_with_descriptions() {
    let def = gqlidl::parse_schema("
    # Autogenerated input type of UpdateTopics
    input UpdateTopicsInput {
      # A unique identifier for the client performing the mutation.
      clientMutationId: String

      # An array of topic names.
      topicNames: [String!]!
    }
    ").unwrap().pop().unwrap();

    assert_eq!("Autogenerated input type of UpdateTopics", def.description().unwrap());
    assert_eq!("input_object", def.typename());
    assert_eq!("UpdateTopicsInput", def.name());

    let mut field = def.fields().unwrap().remove(0);

    assert_eq!("A unique identifier for the client performing the mutation.", field.description().unwrap());
    assert_eq!("clientMutationId", field.name());
    assert_eq!("String", field.typeinfo().name());
    assert_eq!("", field.typeinfo().info());

    field = def.fields().unwrap().remove(1);

    assert_eq!("An array of topic names.", field.description().unwrap());
    assert_eq!("topicNames", field.name());
    assert_eq!("String", field.typeinfo().name());
    assert_eq!("[!]!", field.typeinfo().info());
}
