#include <stdint.h>
#include <stdbool.h>

typedef struct array_of_strings {
  int32_t length;
  const char** values;
} array_and_size;

typedef struct FieldType {
  const char* name;
  const char* type_info;
} FieldType;

typedef struct GraphQLArgument {
  const char* name;
  const char* description;
  FieldType type_info;
} GraphQLArgument;

typedef struct array_of_arguments {
  int32_t length;
  const GraphQLArgument* values;
} array_of_arguments;

typedef struct GraphQLField {
  const char* name;
  const char* description;
  FieldType type_info;
  struct array_of_arguments arguments;
  bool deprecated;
  const char* deprecation_reason;
} GraphQLField;

typedef struct GraphQLValue {
  const char* name;
  const char* description;
} GraphQLValue;

typedef struct array_of_fields {
  int32_t length;
  const GraphQLField* values;
} array_of_fields;

typedef struct array_of_values {
  int32_t length;
  const GraphQLValue* values;
} array_of_values;

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

typedef struct GraphQLEnum {
  const char* name;
  const char* description;
  struct array_of_values values;
} GraphQLEnum;

typedef struct GraphQLInterface {
  const char* name;
  const char* description;
  struct array_of_fields fields;
} GraphQLInterface;

typedef struct GraphQLTypes {
  const char* typename;
  union
  {
    GraphQLScalar scalar;
    GraphQLObject object;
    GraphQLEnum enum_type;
    GraphQLInterface interface;
  };
} GraphQLTypes;

/* This is the actual method exposed by Rust FFI */
uint8_t gqlidl_parse_schema(char* schema, GraphQLTypes** types, size_t* types_len);
