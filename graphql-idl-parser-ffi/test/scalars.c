#include "clar.h"
#include "clar_test.h"
#include "helpers.h"
#include <stdio.h>
#include <string.h>

#include "gql-idl-parser.h"

static char *fixture;

void test_scalars__initialize(void)
{
  global_test_counter++;
}

void test_scalars__cleanup(void)
{
  if (fixture != NULL) {
    free(fixture);
  }
}

void test_scalars__inline(void)
{
  fixture = read_fixture("scalars.graphql");

  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema(fixture, &types, &types_len);

  cl_assert_equal_i(err, 0);

  cl_assert_equal_s(types[0].typename, "scalar");
  cl_assert_equal_s("", types[0].scalar.description);
  cl_assert_equal_s("DateTime", types[0].scalar.name);

  cl_assert_equal_s(types[1].typename, "scalar");
  cl_assert_equal_s("An ISO-8601 encoded UTC date string.", types[1].scalar.description);
  cl_assert_equal_s("DateTime", types[1].scalar.name);
}
