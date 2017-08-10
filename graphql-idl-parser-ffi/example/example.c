#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "gql-idl-parser.h"

int main()
{
  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema("# Yeah yeah\nscalar DateTime\ntype OMG {}", &types, &types_len);

  if (err > 0) {
    printf("Error: Return code %d", err);
    exit(err);
  }

  for (size_t i = 0; i < types_len; i++) {
    printf("typename: %s\n", types[i].typename);
    printf("desc: %s\n", types[i].scalar_type.description);
    printf("name: %s\n", types[i].scalar_type.name);
    if (strncmp(types[i].typename, "scalar", 6) == 0) {
      printf("a scalar!");
    }
    if (strncmp(types[i].typename, "object", 6) == 0) {
      printf("an object!");
    }
  }

  return err;
}
