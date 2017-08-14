static SCALAR: &'static str = "scalar";
static OBJECT: &'static str = "object";
static ENUM: &'static str = "enum";
static INTERFACE: &'static str = "interface";
static UNION: &'static str = "union";
static INPUT_OBJECT: &'static str = "input_object";

pub struct GraphQLScalar {
    description: Option<String>,
    name: String,
}

pub struct GraphQLObject {
    description: Option<String>,
    name: String,
    implements: Option<Vec<String>>,
    directives: Option<Vec<GraphQLDirective>>,
    fields: Option<Vec<GraphQLField>>,
}

pub struct GraphQLEnum {
    description: Option<String>,
    name: String,
    directives: Option<Vec<GraphQLDirective>>,
    values: Vec<GraphQLValue>,
}

pub struct GraphQLInterface {
    description: Option<String>,
    name: String,
    directives: Option<Vec<GraphQLDirective>>,
    fields: Option<Vec<GraphQLField>>,
}

pub struct GraphQLUnion {
    description: Option<String>,
    name: String,
    directives: Option<Vec<GraphQLDirective>>,
    types: Vec<String>,
}

pub struct GraphQLInputObject {
    description: Option<String>,
    name: String,
    directives: Option<Vec<GraphQLDirective>>,
    fields: Option<Vec<GraphQLField>>,
}

pub enum TypeDefinition {
    ScalarType(GraphQLScalar),
    ObjectType(GraphQLObject),
    EnumType(GraphQLEnum),
    InterfaceType(GraphQLInterface),
    UnionType(GraphQLUnion),
    InputObjectType(GraphQLInputObject),
}

#[derive(Clone)]
pub struct GraphQLField {
    description: Option<String>,
    name: String,
    typeinfo: FieldType,
    arguments: Option<Vec<GraphQLArgument>>,
    directives: Option<Vec<GraphQLDirective>>,
}

#[derive(Clone)]
pub struct GraphQLValue {
    description: Option<String>,
    name: String,
    directives: Option<Vec<GraphQLDirective>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphQLArgument {
    description: Option<String>,
    name: String,
    typeinfo: FieldType,
    default: Option<String>,
    directives: Option<Vec<GraphQLDirective>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphQLDirective {
    name: String,
    arguments: Option<Vec<GraphQLDirectiveArgument>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphQLDirectiveArgument {
    name: String,
    value: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldType {
    pub name: String,
    pub info: TypeInfo,
}

impl GraphQLScalar {
    pub fn new(description: Option<String>, name: String) -> GraphQLScalar {
        GraphQLScalar {
            description: description,
            name: name,
        }
    }
}

impl GraphQLObject {
    pub fn new(
        description: Option<String>,
        name: String,
        implements: Option<Vec<String>>,
        directives: Option<Vec<GraphQLDirective>>,
        fields: Option<Vec<GraphQLField>>,
    ) -> GraphQLObject {
        GraphQLObject {
            description: description,
            name: name,
            implements: implements,
            directives: directives,
            fields: fields,
        }
    }
}

impl GraphQLEnum {
    pub fn new(
        description: Option<String>,
        name: String,
        directives: Option<Vec<GraphQLDirective>>,
        values: Vec<GraphQLValue>,
    ) -> GraphQLEnum {
        GraphQLEnum {
            description: description,
            name: name,
            directives: directives,
            values: values,
        }
    }
}

impl GraphQLInterface {
    pub fn new(
        description: Option<String>,
        name: String,
        directives: Option<Vec<GraphQLDirective>>,
        fields: Option<Vec<GraphQLField>>,
    ) -> GraphQLInterface {
        GraphQLInterface {
            description: description,
            name: name,
            directives: directives,
            fields: fields,
        }
    }
}

impl GraphQLUnion {
    pub fn new(
        description: Option<String>,
        name: String,
        directives: Option<Vec<GraphQLDirective>>,
        types: Vec<String>,
    ) -> GraphQLUnion {
        GraphQLUnion {
            description: description,
            name: name,
            directives: directives,
            types: types,
        }
    }
}

impl GraphQLInputObject {
    pub fn new(
        description: Option<String>,
        name: String,
        directives: Option<Vec<GraphQLDirective>>,
        fields: Option<Vec<GraphQLField>>,
    ) -> GraphQLInputObject {
        GraphQLInputObject {
            description: description,
            name: name,
            directives: directives,
            fields: fields,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
            TypeDefinition::ObjectType(GraphQLObject { ref implements, .. }) => implements.clone(),
            _ => panic!("That method does not exist for this type."),
        }
    }

    pub fn directives(&self) -> Option<Vec<GraphQLDirective>> {
        match *self {
            TypeDefinition::ObjectType(GraphQLObject { ref directives, .. }) => directives.clone(),
            TypeDefinition::EnumType(GraphQLEnum { ref directives, .. }) => directives.clone(),
            TypeDefinition::InterfaceType(GraphQLInterface { ref directives, .. }) => {
                directives.clone()
            }
            TypeDefinition::UnionType(GraphQLUnion { ref directives, .. }) => directives.clone(),
            TypeDefinition::InputObjectType(GraphQLInputObject { ref directives, .. }) => {
                directives.clone()
            }
            _ => panic!("That method does not exist for this type."),
        }
    }

    pub fn fields(&self) -> Option<Vec<GraphQLField>> {
        match *self {
            TypeDefinition::ObjectType(GraphQLObject { ref fields, .. }) |
            TypeDefinition::InterfaceType(GraphQLInterface { ref fields, .. }) |
            TypeDefinition::InputObjectType(GraphQLInputObject { ref fields, .. }) => {
                fields.to_owned()
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
impl_graphql_directive_methods! { GraphQLField, GraphQLArgument, GraphQLValue }
impl_graphql_type_methods! { GraphQLField, GraphQLArgument }

impl GraphQLField {
    pub fn new(
        description: Option<String>,
        name: String,
        typeinfo: FieldType,
        arguments: Option<Vec<GraphQLArgument>>,
        directives: Option<Vec<GraphQLDirective>>,
    ) -> GraphQLField {
        GraphQLField {
            description: description,
            name: name,
            typeinfo: typeinfo,
            arguments: arguments,
            directives: directives,
        }
    }

    pub fn arguments(&self) -> Option<Vec<GraphQLArgument>> {
        match self.arguments {
            None => None,
            Some(ref arguments) => Some(arguments.clone()),
        }
    }
}

impl GraphQLArgument {
    pub fn new(
        description: Option<String>,
        name: String,
        typeinfo: FieldType,
        default: Option<String>,
        directives: Option<Vec<GraphQLDirective>>,
    ) -> GraphQLArgument {
        GraphQLArgument {
            description: description,
            name: name,
            typeinfo: typeinfo,
            default: default,
            directives: directives,
        }
    }

    pub fn default(&self) -> Option<&str> {
        self.default.as_ref().map(|s| s.as_ref())
    }
}

impl GraphQLValue {
    pub fn new(
        description: Option<String>,
        name: String,
        directives: Option<Vec<GraphQLDirective>>,
    ) -> GraphQLValue {
        GraphQLValue {
            description: description,
            name: name,
            directives: directives,
        }
    }
}

impl GraphQLDirective {
    pub fn new(name: String, arguments: Option<Vec<GraphQLDirectiveArgument>>) -> GraphQLDirective {
        GraphQLDirective {
            name: name,
            arguments: arguments,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> Option<Vec<GraphQLDirectiveArgument>> {
        match self.arguments {
            None => None,
            Some(ref arguments) => Some(arguments.clone()),
        }
    }
}

impl GraphQLDirectiveArgument {
    pub fn new(name: String, value: Option<String>) -> GraphQLDirectiveArgument {
        GraphQLDirectiveArgument {
            name: name,
            value: value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| s.as_ref())
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
