#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct GraphQLScalar {
  const char* description;
  const char* name;
} GraphQLScalar;

/* This is the actual method exposed by Rust FFI */
uint8_t gqlidl_parse_schema(char* schema, GraphQLScalar** types, size_t* types_len);

int main() {
  GraphQLScalar* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema("# Meow meow \n scalar DateTime", &types, &types_len);

  if (err > 0) {
    printf("Error: Return code %d", err);
    exit(err);
  }

  for (size_t i = 0; i < types_len; i++) {
    printf("desc: %s\n", types[i].description);
    printf("name: %s\n", types[i].name);
  }

  return err;
}
