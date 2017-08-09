#include <stdint.h>
#include <stdbool.h>

typedef struct array_of_strings {
  int32_t length;
  const char** values;
} array_and_size;

typedef struct GraphQLField {
  const char* name;
  const char* description;
  // typeinfo: FieldType,
  // arguments: Vec<GraphQLArgument>,
  bool deprecated;
  const char* deprecation_reason;
} GraphQLField;

typedef struct array_of_fields {
  int32_t length;
  const GraphQLField* values;
} array_of_fields;

typedef struct GraphQLScalar {
  const char* name;
  const char* description;
} GraphQLScalar;

typedef struct GraphQLObject {
  const char* name;
  const char* description;
  struct array_of_strings implements;
  struct array_of_fields fields;
} GraphQLObject;

typedef struct GraphQLTypes {
  const char* typename;
  union
  {
    GraphQLScalar scalar;
    GraphQLObject object;
  };
} GraphQLTypes;

/* This is the actual method exposed by Rust FFI */
uint8_t gqlidl_parse_schema(char* schema, GraphQLTypes** types, size_t* types_len);
