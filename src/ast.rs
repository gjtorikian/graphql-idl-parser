pub struct Definition {
    pub gql_type: GraphQLType,
    pub name: String,
    pub description: String,
    // pub fields: Vec<Field>
}

pub enum GraphQLType {
    ScalarType,
    ObjectType,
    InterfaceType,
    UnionType,
    EnumType,
    InputObjectType,
    // ExtendType,
    // DirectiveType
}

impl GraphQLType {
    pub fn as_str(&self) -> &str {
        match self {
            &GraphQLType::ScalarType => "scalar",
            &GraphQLType::ObjectType => "object",
            &GraphQLType::InterfaceType => "interface",
            &GraphQLType::UnionType => "union",
            &GraphQLType::EnumType => "enum",
            &GraphQLType::InputObjectType => "input_object",
        }
    }
}

pub struct Field {
    // pub gql_type: String,
    pub name: String,
    // pub required: Boolean
}
