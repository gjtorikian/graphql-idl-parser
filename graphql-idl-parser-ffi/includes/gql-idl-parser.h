#ifndef _GQL_IDL_PARSER_H
#define _GQL_IDL_PARSER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct array_of_strings {
  size_t length;
  const char** data;
} array_and_size;

typedef struct FieldType {
  const char* name;
  const char* info;
} FieldType;

typedef struct GraphQLDirectiveArgument {
  const char* name;
  const char* value;
} GraphQLDirectiveArgument;

typedef struct array_of_directive_arguments {
  size_t length;
  const GraphQLDirectiveArgument* data;
} array_of_directive_arguments;

typedef struct GraphQLDirective {
  const char* name;
  struct array_of_directive_arguments arguments;
} GraphQLDirective;

typedef struct array_of_directives {
  size_t length;
  const GraphQLDirective* data;
} array_of_directives;

typedef struct GraphQLArgument {
  const char* description;
  const char* name;
  FieldType type_info;
  const char* default_value;
  struct array_of_directives directives;
} GraphQLArgument;

typedef struct array_of_arguments {
  size_t length;
  const GraphQLArgument* data;
} array_of_arguments;

typedef struct GraphQLField {
  const char* description;
  const char* name;
  FieldType type_info;
  struct array_of_arguments arguments;
  struct array_of_directives directives;
} GraphQLField;

typedef struct array_of_fields {
  size_t length;
  const GraphQLField* data;
} array_of_fields;

typedef struct GraphQLValue {
  const char* description;
  const char* name;
  struct array_of_directives directives;
} GraphQLValue;

typedef struct array_of_values {
  size_t length;
  const GraphQLValue* data;
} array_of_values;

typedef struct GraphQLScalar {
  const char* description;
  const char* name;
} GraphQLScalar;

typedef struct GraphQLObject {
  const char* description;
  const char* name;
  struct array_of_strings implements;
  struct array_of_directives directives;
  struct array_of_fields fields;
} GraphQLObject;

typedef struct GraphQLEnum {
  const char* description;
  const char* name;
  struct array_of_directives directives;
  struct array_of_values values;
} GraphQLEnum;

typedef struct GraphQLInterface {
  const char* description;
  const char* name;
  struct array_of_directives directives;
  struct array_of_fields fields;
} GraphQLInterface;

typedef struct GraphQLUnion {
  const char* description;
  const char* name;
  struct array_of_directives directives;
  struct array_of_strings values;
} GraphQLUnion;

typedef struct GraphQLInputObject {
  const char* description;
  const char* name;
  struct array_of_directives directives;
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
uint8_t gqlidl_parse_schema(const char* schema, GraphQLTypes** types, size_t* types_len);

#ifdef __cplusplus
}
#endif

#endif
