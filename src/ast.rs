pub struct Definition {
    pub description: String,
    pub typename: GraphQLType,
    pub name: String,
    pub implements: Vec<String>,
    pub fields: Vec<Field>,
    pub connections: Vec<Connection>
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
    pub deprecated: bool,
    pub deprecated_reason: String
}

pub struct Connection {
    pub description: String,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub fieldtype: FieldType,
    pub deprecated: bool,
    pub deprecated_reason: String
}

pub struct Argument {
    pub description: String,
    pub name: String,
    pub fieldtype: FieldType,
}

pub struct FieldType {
    pub name: String,
    pub info: TypeInfo,
}

pub enum TypeInfo {
    Nullable,
    NonNullable,
    NullableListNullableElements,
    NullableListNullableNonNullableElements,
    NonNullableListNullableElements,
    NonNullableListNonNullableElements,
}

impl TypeInfo {
    pub fn as_str(&self) -> &str {
        match self {
            &TypeInfo::Nullable => "",
            &TypeInfo::NonNullable => "!",
            &TypeInfo::NullableListNullableElements => "[]",
            &TypeInfo::NullableListNullableNonNullableElements => "[!]",
            &TypeInfo::NonNullableListNullableElements => "[]!",
            &TypeInfo::NonNullableListNonNullableElements => "[!]!",
        }
    }
}
