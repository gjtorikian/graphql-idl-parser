#ifndef _GQL_IDL_PARSER_H
#define _GQL_IDL_PARSER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct array_of_strings {
  int32_t length;
  const char** data;
} array_and_size;

typedef struct FieldType {
  const char* name;
  const char* info;
} FieldType;

typedef struct GraphQLArgument {
  const char* name;
  const char* description;
  FieldType type_info;
} GraphQLArgument;

typedef struct array_of_arguments {
  int32_t length;
  const GraphQLArgument* data;
} array_of_arguments;

typedef struct GraphQLField {
  const char* name;
  const char* description;
  FieldType type_info;
  struct array_of_arguments arguments;
  bool deprecated;
  const char* deprecation_reason;
} GraphQLField;

typedef struct array_of_fields {
  int32_t length;
  const GraphQLField* data;
} array_of_fields;

typedef struct GraphQLValue {
  const char* name;
  const char* description;
} GraphQLValue;

typedef struct array_of_values {
  int32_t length;
  const GraphQLValue* data;
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

typedef struct GraphQLUnion {
  const char* name;
  const char* description;
  struct array_of_strings values;
} GraphQLUnion;

typedef struct GraphQLInputObject {
  const char* name;
  const char* description;
  struct array_of_fields fields;
} GraphQLInputObject;

typedef struct GraphQLTypes {
  const char* typename;
  union {
    GraphQLScalar scalar_type;
    GraphQLObject object_type;
    GraphQLEnum enum_type;
    GraphQLInterface interface_type;
    GraphQLUnion union_type;
    GraphQLInputObject input_object_type;
  };
} GraphQLTypes;

/* This is the actual method exposed by Rust FFI */
uint8_t gqlidl_parse_schema(char* schema, GraphQLTypes** types, size_t* types_len);

#ifdef __cplusplus
}
#endif

#endif
