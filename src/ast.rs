pub struct Definition {
    pub description: String,
    pub typename: GraphQLType,
    pub name: String,
    pub implements: Vec<String>,
    pub fields: Vec<Field>
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
    pub description: String,
    pub name: String,
    pub fieldtype: FieldType,
    pub nullable: bool
}

pub struct FieldType {
    pub name: String,
    pub list: bool,
    pub nullable: bool
}
