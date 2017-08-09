#include <stdint.h>

typedef struct GraphQLScalar {
  const char* description;
  const char* name;
} GraphQLScalar;

typedef struct array_of_strings {
  int32_t length;
  const char** values;
} array_and_size;

typedef struct GraphQLObject {
  const char* description;
  const char* name;
  struct array_of_strings implements;
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
