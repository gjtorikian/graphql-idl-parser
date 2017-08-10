#include "clar.h"
#include "clar_test.h"
#include "helpers.h"
#include <stdio.h>
#include <string.h>

#include "gql-idl-parser.h"

static char *fixture;

void test_interfaces__initialize(void)
{
  global_test_counter++;
}

void test_interfaces__cleanup(void)
{
  if (fixture != NULL) {
    free(fixture);
  }
}

void test_interfaces__inline(void)
{
  fixture = read_fixture("interfaces.graphql");

  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema(fixture, &types, &types_len);

  cl_assert_equal_i(err, 0);

  cl_assert_equal_s(types[0].typename, "interface");
  cl_assert_equal_s("Closable", types[0].interface.name);
  cl_assert_equal_s("An object that can be closed", types[0].interface.description);
  cl_assert_equal_i(1, types[0].interface.fields.length);
  cl_assert_equal_s("closed", types[0].interface.fields.values[0].name);
  cl_assert_equal_s("`true` if the object is closed (definition of closed may depend on type)", types[0].interface.fields.values[0].description);
  cl_assert_equal_s("Boolean", types[0].interface.fields.values[0].type_info.name);
  cl_assert_equal_s("!", types[0].interface.fields.values[0].type_info.type_info);

}
