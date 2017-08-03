#include <stdint.h>

typedef struct GraphQLScalar {
  const char* description;
  const char* name;
} GraphQLScalar;

typedef struct GraphQLObject {
  const char* description;
  const char* name;
  // const char* implements;
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
