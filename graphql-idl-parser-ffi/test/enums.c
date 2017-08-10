#include "clar.h"
#include "clar_test.h"
#include "helpers.h"
#include <stdio.h>
#include <string.h>

#include "gql-idl-parser.h"

static char *fixture;

void test_enums__initialize(void)
{
  global_test_counter++;
}

void test_enums__cleanup(void)
{
  if (fixture != NULL) {
    free(fixture);
  }
}

void test_enums__inline(void)
{
  fixture = read_fixture("enums.graphql");

  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema(fixture, &types, &types_len);

  cl_assert_equal_i(err, 0);

  cl_assert_equal_s(types[0].typename, "enum");
  cl_assert_equal_s("ProjectState", types[0].enum_type.name);
  cl_assert_equal_s("State of the project; either 'open' or 'closed'", types[0].enum_type.description);
  cl_assert_equal_i(2, types[0].enum_type.values.length);
  cl_assert_equal_s("CLOSED", types[0].enum_type.values.values[0].name);
  cl_assert_equal_s("The project is closed.", types[0].enum_type.values.values[0].description);
  cl_assert_equal_s("OPEN", types[0].enum_type.values.values[1].name);
  cl_assert_equal_s("The project is open.", types[0].enum_type.values.values[1].description);
}
