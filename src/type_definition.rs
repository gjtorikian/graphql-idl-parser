static SCALAR: &'static str = "scalar";
static OBJECT: &'static str = "object";
static ENUM: &'static str = "enum";
static INTERFACE: &'static str = "interface";
static UNION: &'static str = "union";
static INPUT_OBJECT: &'static str = "input_object";

fn convert_description(description: Vec<String>) -> Option<String> {
    if description.len() > 0 {
        return Some(description.join(" "));
    } else {
        None
    }
}

pub fn check_deprecated(e: Option<bool>) -> bool {
    match e {
        None => false,
        Some(_) => true,
    }
}

pub struct GraphQLScalar {
    description: Option<String>,
    name: String,
}

pub struct GraphQLObject {
    description: Option<String>,
    name: String,
    implements: Option<Vec<String>>,
    fields: Vec<GraphQLField>,
}

pub struct GraphQLEnum {
    description: Option<String>,
    name: String,
    values: Vec<GraphQLValue>,
}

pub struct GraphQLInterface {
    description: Option<String>,
    name: String,
    fields: Vec<GraphQLField>,
}

pub struct GraphQLUnion {
    description: Option<String>,
    name: String,
    types: Vec<String>,
}

pub struct GraphQLInputObject {
    description: Option<String>,
    name: String,
    fields: Vec<GraphQLField>,
}

#[derive(Clone)]
pub struct GraphQLField {
    description: Option<String>,
    name: String,
    typeinfo: FieldType,
    arguments: Option<Vec<GraphQLArgument>>,
    deprecated: bool,
    deprecation_reason: Option<String>,
}

#[derive(Clone)]
pub struct GraphQLValue {
    name: String,
    description: Option<String>,
}

#[derive(Clone)]
pub struct GraphQLArgument {
    description: Option<String>,
    name: String,
    typeinfo: FieldType,
}

#[derive(Clone)]
pub struct FieldType {
    pub name: String,
    pub info: TypeInfo,
}

pub enum TypeDefinition {
    ScalarType(GraphQLScalar),
    ObjectType(GraphQLObject),
    EnumType(GraphQLEnum),
    InterfaceType(GraphQLInterface),
    UnionType(GraphQLUnion),
    InputObjectType(GraphQLInputObject),
}

impl GraphQLScalar {
    pub fn new(description: Vec<String>, name: String) -> GraphQLScalar {
        GraphQLScalar {
            description: convert_description(description),
            name: name,
        }
    }
}

impl GraphQLObject {
    pub fn new(
        description: Vec<String>,
        name: String,
        implements: Option<Vec<String>>,
        fields: Vec<GraphQLField>,
    ) -> GraphQLObject {
        GraphQLObject {
            description: convert_description(description),
            name: name,
            implements: implements,
            fields: fields,
        }
    }
}

impl GraphQLEnum {
    pub fn new(
        description: Vec<String>,
        name: String,
        values: Vec<GraphQLValue>,
    ) -> GraphQLEnum {
        GraphQLEnum {
            description: convert_description(description),
            name: name,
            values: values,
        }
    }
}

impl GraphQLInterface {
    pub fn new(
        description: Vec<String>,
        name: String,
        fields: Vec<GraphQLField>,
    ) -> GraphQLInterface {
        GraphQLInterface {
            description: convert_description(description),
            name: name,
            fields: fields,
        }
    }
}

impl GraphQLUnion {
    pub fn new(description: Vec<String>, name: String, types: Vec<String>) -> GraphQLUnion {
        GraphQLUnion {
            description: convert_description(description),
            name: name,
            types: types,
        }
    }
}

impl GraphQLInputObject {
    pub fn new(
        description: Vec<String>,
        name: String,
        fields: Vec<GraphQLField>,
    ) -> GraphQLInputObject {
        GraphQLInputObject {
            description: convert_description(description),
            name: name,
            fields: fields,
        }
    }
}


#[derive(Clone)]
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

impl TypeDefinition {
    pub fn typename(&self) -> &str {
        match *self {
            TypeDefinition::ScalarType { .. } => SCALAR,
            TypeDefinition::ObjectType { .. } => OBJECT,
            TypeDefinition::EnumType { .. } => ENUM,
            TypeDefinition::InterfaceType { .. } => INTERFACE,
            TypeDefinition::UnionType { .. } => UNION,
            TypeDefinition::InputObjectType { .. } => INPUT_OBJECT,
        }
    }

    impl_graphql_objects_common_methods! {
        ScalarType:GraphQLScalar,
        ObjectType:GraphQLObject,
        EnumType:GraphQLEnum,
        InterfaceType:GraphQLInterface,
        UnionType:GraphQLUnion,
        InputObjectType:GraphQLInputObject
    }

    pub fn implements(&self) -> Option<Vec<String>> {
        match *self {
            TypeDefinition::ObjectType(GraphQLObject { ref implements, .. }) => {
                implements.clone()
            }
            _ => panic!("That method does not exist for this type."),
        }
    }

    pub fn fields(&self) -> Option<Vec<GraphQLField>> {
        match *self {
            TypeDefinition::ObjectType(GraphQLObject { ref fields, .. }) |
            TypeDefinition::InterfaceType(GraphQLInterface { ref fields, .. }) |
            TypeDefinition::InputObjectType(GraphQLInputObject { ref fields, .. }) => {
                if fields.len() > 0 {
                    return Some(fields.to_vec());
                }
                None
            }
            _ => panic!("That method does not exist for this type."),
        }
    }

    pub fn values(&self) -> Option<Vec<GraphQLValue>> {
        match *self {
            TypeDefinition::EnumType(GraphQLEnum { ref values, .. }) => {
                if values.len() > 0 {
                    return Some(values.to_vec());
                }
                None
            }
            _ => panic!("That method does not exist for this type."),
        }
    }

    pub fn types(&self) -> Option<Vec<String>> {
        match *self {
            TypeDefinition::UnionType(GraphQLUnion { ref types, .. }) => {
                if types.len() > 0 {
                    return Some(types.to_vec());
                }
                None
            }
            _ => panic!("That method does not exist for this type."),
        }
    }
}

impl_graphql_meta_methods! { GraphQLField, GraphQLArgument, GraphQLValue }


impl_graphql_deprecated_methods! { GraphQLField }



impl_graphql_type_methods! { GraphQLField, GraphQLArgument }

impl GraphQLField {
    pub fn new(
        description: Vec<String>,
        name: String,
        typeinfo: FieldType,
        arguments: Option<Vec<GraphQLArgument>>,
        deprecated: bool,
        deprecation_reason: Option<String>,
    ) -> GraphQLField {
        GraphQLField {
            description: convert_description(description),
            name: name,
            typeinfo: typeinfo,
            arguments: arguments,
            deprecated: deprecated,
            deprecation_reason: deprecation_reason,
        }
    }

    pub fn arguments(&self) -> Option<Vec<GraphQLArgument>> {
        match self.arguments {
            None => None,
            Some(ref arguments) => self.arguments.clone()
        }
    }
}

impl GraphQLArgument {
    pub fn new(description: Vec<String>, name: String, typeinfo: FieldType) -> GraphQLArgument {
        GraphQLArgument {
            description: convert_description(description),
            name: name,
            typeinfo: typeinfo,
        }
    }
}

impl GraphQLValue {
    pub fn new(description: Vec<String>, name: String) -> GraphQLValue {
        GraphQLValue {
            description: convert_description(description),
            name: name,
        }
    }
}

impl FieldType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn info(&self) -> &str {
        &self.info.as_str()
    }
}
