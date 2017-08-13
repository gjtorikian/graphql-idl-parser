macro_rules! impl_graphql_meta_methods {
    ($($type_: ty),*) => {
      $(
        impl $type_ {
            pub fn description(&self) -> Option<&str> { self.description.as_ref().map(|s| s.as_ref()) }

            pub fn name(&self) -> &str { self.name.as_ref() }
        }
      )*
    };
}

macro_rules! impl_graphql_directive_methods {
    ($($type_: ty),*) => {
      $(
        impl $type_ {
            pub fn directives(&self) -> Option<Vec<GraphQLDirective>> {
                self.directives.clone()
            }
        }
      )*
    };
}

macro_rules! impl_graphql_type_methods {
    ($($type_: ty),*) => {
      $(
        impl $type_ {
            pub fn typeinfo(&self) -> FieldType {
                self.typeinfo.to_owned()
            }
        }
      )*
    };
}

macro_rules! impl_graphql_objects_common_methods {
    (
        $(
            $x:ident:$y:ident
        ),*
    ) => {
        pub fn description(&self) -> Option<&str> {
            match *self {
                $(
                    TypeDefinition::$x($y{ ref description, .. }) => {
                        description.as_ref().map(|s| s.as_ref())
                    }
                ),*
            }
        }

        pub fn name(&self) -> &str {
            match *self {
                $(
                    TypeDefinition::$x($y{ ref name, .. }) => &name
                ),*
            }
        }
    }
}
