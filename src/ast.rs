static SCALAR: &'static str = "scalar";
static OBJECT: &'static str = "object";
static ENUM: &'static str = "enum";

pub struct GraphQLScalar {
    description: Option<String>,
    name: String
}

pub struct GraphQLObject {
    description: Option<String>,
    name: String,
    implements: Vec<String>,
    fields: Vec<GraphQLField>,
}

pub struct GraphQLEnum {
    description: Option<String>,
    name: String,
    values: Vec<GraphQLValue>,
}

#[derive(Clone)]
pub struct GraphQLField {
  description: Option<String>,
  name: String,
  typeinfo: FieldType,
  arguments: Vec<GraphQLArgument>,
  deprecated: bool,
  deprecation_reason: Option<String>,
}

#[derive(Clone)]
pub struct GraphQLValue {
  description: Option<String>,
  name: String
}

#[derive(Clone)]
pub struct GraphQLArgument {
    description: Option<String>,
    name: String,
    typeinfo: FieldType,
}

pub enum GraphQLType {
    ScalarType(GraphQLScalar),
    ObjectType(GraphQLObject),
    EnumType(GraphQLEnum),
}

impl GraphQLScalar {
    pub fn new(description: Option<String>, name: String) -> GraphQLScalar {
      GraphQLScalar {
          description: description,
          name: name
      }
    }
}

impl GraphQLObject {
    pub fn new(description: Option<String>, name: String, implements: Vec<String>, fields: Vec<GraphQLField>) -> GraphQLObject {
      GraphQLObject {
          description: description,
          name: name,
          implements: implements,
          fields: fields
      }
    }
}

impl GraphQLEnum {
    pub fn new(description: Option<String>, name: String, values: Vec<GraphQLValue>) -> GraphQLEnum {
      GraphQLEnum {
          description: description,
          name: name,
          values: values
      }
    }
}

#[derive(Clone)]
pub struct FieldType {
    pub name: String,
    pub info: TypeInfo,
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

impl GraphQLType {
    pub fn typename(&self) -> &str {
        match *self {
            GraphQLType::ScalarType { .. } => SCALAR,
            GraphQLType::ObjectType { .. } => OBJECT,
            GraphQLType::EnumType { .. } => ENUM,
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            GraphQLType::ScalarType(GraphQLScalar{ ref name, .. }) => &name,
            GraphQLType::ObjectType(GraphQLObject{ ref name, .. }) => &name,
            GraphQLType::EnumType(GraphQLEnum{ ref name, .. }) => &name,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match *self {
            GraphQLType::ScalarType(GraphQLScalar{ ref description, .. }) => description.as_ref().map(|s| s.as_ref()),
            GraphQLType::ObjectType(GraphQLObject{ ref description, .. }) => description.as_ref().map(|s| s.as_ref()),
            GraphQLType::EnumType(GraphQLEnum{ ref description, .. }) => description.as_ref().map(|s| s.as_ref()),
        }
    }

    pub fn implements(&self) -> Option<Vec<String>> {
        match *self {
            GraphQLType::ObjectType(GraphQLObject{ ref implements, .. }) => {
                if implements.len() > 0 {
                    return Some(implements.to_vec())
                }
                None
            },
            _ => panic!("That method does not exist for this type.")
        }
    }

    pub fn fields(&self) -> Option<Vec<GraphQLField>> {
        match *self {
            GraphQLType::ObjectType(GraphQLObject{ ref fields, .. }) => {
                if fields.len() > 0 {
                    return Some(fields.to_vec())
                }
                None
            },
            _ => panic!("That method does not exist for this type.")
        }
    }

    pub fn values(&self) -> Option<Vec<GraphQLValue>> {
        match *self {
            GraphQLType::EnumType(GraphQLEnum{ ref values, .. }) => {
                if values.len() > 0 {
                    return Some(values.to_vec())
                }
                None
            },
            _ => panic!("That method does not exist for this type.")
        }
    }
}

impl GraphQLField {
    pub fn new(description: Option<String>, name: String, typeinfo: FieldType, arguments: Vec<GraphQLArgument>, deprecated: bool, deprecation_reason: Option<String>) -> GraphQLField {
      GraphQLField {
          description: description,
          name: name,
          typeinfo: typeinfo,
          arguments: arguments,
          deprecated: deprecated,
          deprecation_reason: deprecation_reason
      }
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_ref())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typeinfo(&self) -> FieldType {
        self.typeinfo.to_owned()
    }

    pub fn arguments(&self) -> Option<Vec<GraphQLArgument>> {
        if self.arguments.len() > 0 {
            return Some(self.arguments.to_vec())
        }
        None
    }

    pub fn deprecated(&self) -> bool {
        self.deprecated
    }

    pub fn deprecation_reason(&self) -> Option<&str> {
        self.deprecation_reason.as_ref().map(|s| s.as_ref())
    }
}

impl GraphQLArgument {
    pub fn new(description: Option<String>, name: String, typeinfo: FieldType) -> GraphQLArgument {
      GraphQLArgument {
          description: description,
          name: name,
          typeinfo: typeinfo
      }
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_ref())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typeinfo(&self) -> FieldType {
        self.typeinfo.to_owned()
    }
}

impl GraphQLValue {
    pub fn new(description: Option<String>, name: String) -> GraphQLValue {
      GraphQLValue {
          description: description,
          name: name
      }
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_ref())
    }

    pub fn name(&self) -> &str {
        &self.name
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
